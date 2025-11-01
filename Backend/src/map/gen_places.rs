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

#[derive(Debug, Deserialize)]
struct DirectionsResponse {
    routes: Vec<DirectionsRoute>,
}

#[derive(Debug, Deserialize)]
struct DirectionsRoute {
    overview_polyline: Polyline,
}

#[derive(Debug, Deserialize)]
struct Polyline {
    points: String,
}

// Decode Google’s encoded polyline format → Vec<(lat, lng)>
fn decode_polyline(encoded: &str) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut index = 0;
    let mut lat = 0i64;
    let mut lng = 0i64;

    while index < encoded.len() {
        let mut b;
        let mut shift = 0;
        let mut result = 0;
        loop {
            b = encoded.as_bytes()[index] as i64 - 63;
            index += 1;
            result |= (b & 0x1F) << shift;
            shift += 5;
            if b < 0x20 {
                break;
            }
        }
        let dlat = if (result & 1) != 0 { !(result >> 1) } else { result >> 1 };
        lat += dlat;

        shift = 0;
        result = 0;
        loop {
            b = encoded.as_bytes()[index] as i64 - 63;
            index += 1;
            result |= (b & 0x1F) << shift;
            shift += 5;
            if b < 0x20 {
                break;
            }
        }
        let dlng = if (result & 1) != 0 { !(result >> 1) } else { result >> 1 };
        lng += dlng;

        points.push((lat as f64 / 1e5, lng as f64 / 1e5));
    }
    points
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
    candidates: &[(String, (f64, f64))],
    min_distance_m: f64,
    max_n: usize,
) -> Vec<Location> {
    let mut chosen = Vec::new();
    for (name, loc) in candidates {
        if chosen.iter().all(|c: &Location| haversine_distance(c.location, *loc) >= min_distance_m)
        {
            chosen.push(Location {
                name: name.clone(),
                location: *loc,
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
        let candidates: Vec<(String, (f64, f64))> = all_results
            .into_iter()
            .map(|p| {
                (
                    format!("{} ({})", p.name, loc_type),
                    (p.geometry.location.lat, p.geometry.location.lng),
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
        (acc.0 + loc.location.0, acc.1 + loc.location.1)
    });
    let centroid = (sum_lat / locations.len() as f64, sum_lng / locations.len() as f64);

    let max_dist = locations
        .iter()
        .map(|loc| {
            let dx = loc.location.0 - centroid.0;
            let dy = loc.location.1 - centroid.1;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(0.0, f64::max)
        .max(1e-9);
    let scale = 0.9 / max_dist;

    // Step 4: Fetch routes between sequential locations
    let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();
    for i in 0..locations.len().saturating_sub(1) {
        let origin = locations[i].location;
        let dest = locations[i + 1].location;
        let directions_url = format!(
            "https://maps.googleapis.com/maps/api/directions/json?origin={},{}&destination={},{}&mode=driving&key={}",
            origin.0, origin.1, dest.0, dest.1, api_key
        );

        let dir_res: DirectionsResponse = client.get(&directions_url).send().await?.json().await?;
        if let Some(route) = dir_res.routes.get(0) {
            let decoded = decode_polyline(&route.overview_polyline.points);
            routes.push(decoded);
        }
    }

    // Step 5: Transform coordinates for map visualization
    let transform_point = |(lat, lng): (f64, f64)| {
        let x = (lat - centroid.0) * scale;
        let y = (lng - centroid.1) * scale;
        let (x, y) = (y, -x); // rotate 90° CCW
        (x, -y) // mirror vertically
    };

    let transformed_locations: Vec<Location> = locations
        .into_iter()
        .map(|loc| Location {
            name: loc.name,
            location: transform_point(loc.location),
        })
        .collect();

    let transformed_routes: Vec<Vec<(f64, f64)>> = routes
        .into_iter()
        .map(|route| {
            let transformed: Vec<(f64, f64)> =
                route.into_iter().map(transform_point).collect();
            interpolate_points(&transformed, 4)
        })
        .collect();

    Ok(Map {
        locations: transformed_locations,
        routes: transformed_routes,
    })
}

// Downsample polyline
fn interpolate_points(points: &[(f64, f64)], n: usize) -> Vec<(f64, f64)> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    result.push(points[0]);
    for i in (1..points.len() - 1).step_by(n) {
        result.push(points[i]);
    }
    if points.len() > 1 {
        result.push(points[points.len() - 1]);
    }
    result
}
