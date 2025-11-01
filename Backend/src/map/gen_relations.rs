use crate::types::{Character, Relations, Relation};
use std::collections::HashMap;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn gen_relations(characters: Vec<Character>) -> Relations {
    let mut rng = thread_rng();
    let mut names: Vec<String> = characters.iter().map(|c| c.name.clone()).collect();
    names.shuffle(&mut rng);

    let mut relations: Relations = HashMap::new();

    // Track which characters already got a relation
    let mut assigned = std::collections::HashSet::new();

    for i in 0..names.len() {
        if assigned.contains(&names[i]) {
            continue;
        }

        // Pick another character randomly who doesn't yet have a relation
        let others: Vec<_> = names.iter()
            .filter(|&&ref n| n != &names[i] && !assigned.contains(n))
            .cloned()
            .collect();

        if others.is_empty() {
            break; // no one left to pair
        }

        let other = others.choose(&mut rng).unwrap().clone();

        // Randomly choose a relation type
        let rel_type = match rand::random::<u8>() % 3 {
            0 => Relation::Sibling,
            1 => Relation::Parent,
            _ => Relation::Friend,
        };

        match rel_type {
            Relation::Sibling => {
                relations.entry(names[i].clone()).or_default().push((other.clone(), Relation::Sibling));
                relations.entry(other.clone()).or_default().push((names[i].clone(), Relation::Sibling));
            },
            Relation::Parent => {
                // Randomly assign parent/child direction
                if rand::random::<bool>() {
                    relations.entry(names[i].clone()).or_default().push((other.clone(), Relation::Parent));
                    relations.entry(other.clone()).or_default().push((names[i].clone(), Relation::Child));
                } else {
                    relations.entry(names[i].clone()).or_default().push((other.clone(), Relation::Child));
                    relations.entry(other.clone()).or_default().push((names[i].clone(), Relation::Parent));
                }
            },
            Relation::Friend => {
                relations.entry(names[i].clone()).or_default().push((other.clone(), Relation::Friend));
                relations.entry(other.clone()).or_default().push((names[i].clone(), Relation::Friend));
            },
            _ => {}
        }

        assigned.insert(names[i].clone());
        assigned.insert(other.clone());
    }

    // For any leftover character without a relation, assign a friend to someone random
    let unassigned: Vec<_> = names.into_iter().filter(|n| !relations.contains_key(n)).collect();
    for n in unassigned {
        // Pick someone randomly
        let keys: Vec<_> = relations.keys().cloned().collect();
        if let Some(other) = keys.choose(&mut rng) {
            relations.entry(n.clone()).or_default().push((other.clone(), Relation::Friend));
            relations.entry(other.clone()).or_default().push((n.clone(), Relation::Friend));
        }
    }

    relations
}
