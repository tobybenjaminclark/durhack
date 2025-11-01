use crate::viz_map;
use crate::fetch_map;
use std::fs::File;
use std::io::BufReader;
use dotenvy::dotenv;
use rand::prelude::IndexedRandom;
use serde_json::json;
use crate::io::io::{read_map_from_file, write_map_to_file};
use crate::map::gen_characters::gen_characters;
use crate::map::gen_relations::gen_relations;

pub async fn init_map(name: String, live: bool) -> String {

    let map = {
        if live {
            dotenv().ok();
            println!("Fetching up to {} attractions in {}...", 5, name);
            // Destructure the returned tuple
            let map = fetch_map(&*name, 10, 200.0).await.unwrap();
            viz_map(&map.clone());
            println!("{}", map);



            let _ = write_map_to_file(&map, "map.json");

            map
        }
        else {
            read_map_from_file("map.json").unwrap()
        }
    };

    let characters = gen_characters(5, map.clone());
    for c in characters.clone() {
        println!("{}", c);
    }

    let relations = gen_relations(characters);
    println!("{:#?}", relations);

    // Build JSON
    let json_output = json!({
        "INIT_MAP": {
            "map": map,
        }
    });

    // Convert to string
    json_output.to_string()

}

