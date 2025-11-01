use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString, Display};

#[derive(AsRefStr, EnumString, Display, Debug)]
pub enum LocEnum {
    Restaurant,
    Hotel,
    School,
    Church,
    Bank,
    Gym
}


#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub location: (f64, f64),
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.location == other.location
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.location.0.to_bits().hash(state); // f64 cannot be hashed directly
        self.location.1.to_bits().hash(state);
    }
}


impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:.5}, {:.5})", self.name, self.location.0, self.location.1)
    }
}

#[derive(Clone)]
pub struct Map {
    pub locations: Vec<Location>,
    pub routes: Vec<Vec<(f64, f64)>>
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Locations:")?;
        for (i, place) in self.locations.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, place)?;
        }
        writeln!(f, "Routes: {}", self.routes.len())?;
        for (i, route) in self.routes.iter().enumerate() {
            // Take first 5 points or fewer
            let first_points: Vec<String> = route.iter()
                .take(5)
                .map(|(x, y)| format!("({:.5}, {:.5})", x, y))
                .collect();
            writeln!(
                f,
                "  Route {} ({} points, first 5: [{}])",
                i + 1,
                route.len(),
                first_points.join(", ")
            )?;
        }
        Ok(())
    }
}
