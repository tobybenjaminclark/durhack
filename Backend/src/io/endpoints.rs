use crate::fetch_map;
use std::fs::File;
use std::io::BufReader;
use dotenvy::dotenv;
use rand::prelude::IndexedRandom;
use serde_json::json;
use crate::io::io::{read_map_from_file, write_map_to_file};

pub async fn init_map(name: String, live: bool) -> String {

    let map = {
        if live {
            dotenv().ok();
            println!("Fetching up to {} attractions in {}...", 5, name);
            // Destructure the returned tuple
            let map = fetch_map(&*name, 10, 100.0).await.unwrap();
            println!("{}", map);

            let _ = write_map_to_file(&map, "map.json");

            map
        }
        else {
            read_map_from_file("map.json").unwrap()
        }
    };

    // Build JSON
    let json_output = json!({
        "INIT_MAP": {
            "map": map,
        }
    });

    // Convert to string
    json_output.to_string()

}

