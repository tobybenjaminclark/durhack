use std::collections::HashSet;
use dotenvy::dotenv;
use crate::types::{Map, Location, LocEnum};
use serde::Deserialize;
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct GeocodeResponse {
    results: Vec<GeocodeResult>,
}

#[derive(Debug, Deserialize)]
struct GeocodeResult {
    geometry: Geometry,
    formatted_address: String,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    location: LatLng,
}

#[derive(Debug, Deserialize)]
struct LatLng {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Deserialize)]
struct PlacesResponse {
    results: Vec<PlaceResult>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlaceResult {
    name: String,
    geometry: Geometry,
}

// Function to check if two line segments intersect
fn lines_intersect(a: (f64, f64), b: (f64, f64), c: (f64, f64), d: (f64, f64)) -> bool {
    fn ccw(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> bool {
        (p3.1 - p1.1) * (p2.0 - p1.0) > (p2.1 - p1.1) * (p3.0 - p1.0)
    }
    (ccw(a, c, d) != ccw(b, c, d)) && (ccw(a, b, c) != ccw(a, b, d))
}

// Normalize route ordering to avoid duplicates
fn normalize_route(a: &Location, b: &Location) -> (Location, Location) {
    if a.name < b.name {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    }
}

fn haversine_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let r = 6371000.0;
    let lat1 = a.0.to_radians();
    let lat2 = b.0.to_radians();
    let dlat = (b.0 - a.0).to_radians();
    let dlng = (b.1 - a.1).to_radians();
    let hav = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlng / 2.0).sin().powi(2);
    2.0 * r * hav.sqrt().asin()
}

/// Select points that are at least `min_distance_m` apart
fn pick_spread_out(
    candidates: &[(String, (f64, f64), LocEnum)],
    min_distance_m: f64,
    max_n: usize,
) -> Vec<Location> {
    let mut chosen = Vec::new();
    for (name, loc, _type) in candidates {
        if chosen.iter().all(|c: &Location| haversine_distance(c.coords, *loc) >= min_distance_m)
        {
            chosen.push(Location {
                name: name.clone(),
                coords: *loc,
                _type: _type.clone()
            });
        }
        if chosen.len() >= max_n {
            break;
        }
    }
    chosen
}

pub async fn fetch_map(
    place: &str,
    n: usize,
    min_distance_m: f64,
) -> Result<Map, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let client = Client::new();

    // Step 1: Geocode starting place
    let geo_url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        urlencoding::encode(place),
        api_key
    );
    let geo_res: GeocodeResponse = client.get(&geo_url).send().await?.json().await?;
    let first_result = geo_res
        .results
        .get(0)
        .ok_or("No results found for that place")?;
    let center = (
        first_result.geometry.location.lat,
        first_result.geometry.location.lng,
    );

    // Step 2: Find places from multiple types (LocEnum)
    let mut locations = Vec::new();
    let types = [
        LocEnum::Restaurant,
        LocEnum::Hotel,
        LocEnum::School,
        LocEnum::Church,
        LocEnum::Bank,
        LocEnum::Gym,
    ];
    let per_type = (n / types.len()).max(1);

    for loc_type in types {
        let type_str = loc_type.as_ref().to_lowercase();
        let mut radius = 2000;
        let mut all_results = Vec::new();

        // Fetch up to 3 pages, expanding radius if needed
        for _ in 0..3 {
            let places_url = format!(
                "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius={}&type={}&key={}",
                center.0, center.1, radius, type_str, api_key
            );

            let mut res: PlacesResponse = client.get(&places_url).send().await?.json().await?;
            all_results.extend(res.results);

            // Handle pagination
            let mut page_token = res.next_page_token.take();
            while let Some(token) = page_token {
                tokio::time::sleep(Duration::from_secs(2)).await; // Google requires delay
                let next_url = format!(
                    "https://maps.googleapis.com/maps/api/place/nearbysearch/json?pagetoken={}&key={}",
                    token, api_key
                );
                let next_res: PlacesResponse = client.get(&next_url).send().await?.json().await?;
                all_results.extend(next_res.results);
                page_token = next_res.next_page_token;
                if all_results.len() >= 60 {
                    break;
                }
            }

            if all_results.len() >= per_type {
                break;
            }

            radius *= 2; // expand search area
        }

        if all_results.is_empty() {
            continue;
        }

        // Apply spacing filter
        let candidates: Vec<(String, (f64, f64), LocEnum)> = all_results
            .into_iter()
            .filter(|p| {
                p.name.to_lowercase() != place.to_lowercase()
                    && haversine_distance(
                    (p.geometry.location.lat, p.geometry.location.lng),
                    center
                ) > 10.0
            })
            .map(|p| {
                (
                    p.name,
                    (p.geometry.location.lat, p.geometry.location.lng),
                    loc_type.clone(),
                )
            })
            .collect();

        let spread = pick_spread_out(&candidates, min_distance_m, per_type);
        locations.extend(spread);
    }

    if locations.is_empty() {
        return Err("No locations found".into());
    }

    // Step 3: Compute centroid & scale for normalization
    let (sum_lat, sum_lng) = locations.iter().fold((0.0, 0.0), |acc, loc| {
        (acc.0 + loc.coords.0, acc.1 + loc.coords.1)
    });
    let centroid = (sum_lat / locations.len() as f64, sum_lng / locations.len() as f64);

    let max_dist = locations
        .iter()
        .map(|loc| {
            let dx = loc.coords.0 - centroid.0;
            let dy = loc.coords.1 - centroid.1;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(0.0, f64::max)
        .max(1e-9);
    let scale = 0.9 / max_dist;

    // Step 4: Transform coordinates for map visualization
    let transform_point = |(lat, lng): (f64, f64)| {
        let x = (lat - centroid.0) * scale;
        let y = (lng - centroid.1) * scale;
        let (x, y) = (y, -x); // rotate 90° CCW
        (x, -y) // mirror vertically
    };

    let mut seen_names = HashSet::new();
    let mut seen_coords: HashSet<(i64, i64)> = HashSet::new();
    let mut transformed_locations: Vec<Location> = Vec::new();

    for loc in locations {
        let coords = transform_point(loc.coords);

        // Convert coordinates to integers for hashing
        let key = (
            (coords.0 * 1_000_000.0).round() as i64,
            (coords.1 * 1_000_000.0).round() as i64,
        );

        if seen_names.contains(&loc.name) || seen_coords.contains(&key) {
            continue;
        }

        seen_names.insert(loc.name.clone());
        seen_coords.insert(key);

        transformed_locations.push(Location {
            name: loc.name,
            coords,
            _type: loc._type,
        });
    }

    // Step 5: Generate a full mesh of routes
    let mut routes = Vec::new();
    for i in 0..transformed_locations.len() {
        for j in (i + 1)..transformed_locations.len() {
            routes.push(normalize_route(
                &transformed_locations[i],
                &transformed_locations[j],
            ));
        }
    }

    // Step 6: Remove overlapping routes
    let mut non_overlapping: Vec<(_, _)> = Vec::<(Location, Location)>::new();
    for (a, b) in routes {
        if !non_overlapping.iter().any(|(c, d): &(_, _)| lines_intersect(a.coords, b.coords, c.coords, d.coords)) {
            non_overlapping.push((a, b));
        }
    }

    Ok(Map {
        locations: transformed_locations,
        routes: non_overlapping,
    })

}
