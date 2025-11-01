use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString, Display};

#[derive(AsRefStr, EnumString, Display, Debug, Clone, Serialize, Deserialize)]
pub enum LocEnum {
    Restaurant,
    Hotel,
    School,
    Church,
    Bank,
    Gym
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub coords: (f64, f64),
    pub _type: LocEnum,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.coords == other.coords
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.coords.0.to_bits().hash(state); // f64 cannot be hashed directly
        self.coords.1.to_bits().hash(state);
    }
}


impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:.5}, {:.5})", self.name, self.coords.0, self.coords.1)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub locations: Vec<Location>,
    pub routes: Vec<(Location, Location)>
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Locations:")?;
        for (i, place) in self.locations.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, place)?;
        }
        Ok(())
    }
}
