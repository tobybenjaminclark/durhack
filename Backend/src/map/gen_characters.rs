use std::cmp::PartialEq;
use rand::prelude::IndexedRandom;
use crate::types::{Character, Map, Location, Role, LocEnum, role_to_loc, loc_to_role};
use fake::{Fake, faker::name::en::Name};
use rand::{seq::SliceRandom, Rng};


pub fn gen_characters(n: i32, map: Map) -> Vec<Character> {
    let mut rng = rand::thread_rng();
    let mut characters = Vec::new();

    // Determine which roles are possible based on the map
    let available_loc_types: Vec<LocEnum> = map.locations.iter().map(|l| l._type.clone()).collect();
    let mut available_roles = Vec::new();
    for loc_type in &available_loc_types {
        available_roles.push(loc_to_role(loc_type.clone()));
    }

    if available_roles.is_empty() {
        eprintln!("⚠️ No valid roles found for this map.");
        return Vec::new();
    }

    for _ in 0..n {
        let name: String = Name().fake();
        let role = available_roles.choose(&mut rng).unwrap().clone();
        let preferred_loc_type = role_to_loc(role.clone());

        let mut selected_locations = Vec::new();
        for _ in 0..7 {
            let location = map
                .locations
                .choose_weighted(&mut rng, |loc| {
                    if loc._type == preferred_loc_type {
                        3.0
                    } else {
                        1.0
                    }
                })
                .unwrap()
                .clone();
            selected_locations.push(location);
        }

        characters.push(Character {
            name,
            role,
            locations: selected_locations,
            is_murderer: false,
        });
    }

    if !characters.is_empty() {
        let murderer_index = rng.gen_range(0..characters.len());
        characters[murderer_index].is_murderer = true;
    }

    characters
}
