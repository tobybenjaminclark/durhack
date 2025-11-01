use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString, Display};
use crate::types::LocEnum::{Bank, Church, Gym, Hotel, Restaurant, School};
use crate::types::Role::{Bankteller, Chef, Janitor, Priest, Teacher, Trainer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Relation {
    Parent,
    Child,
    Sibling,
    Friend
}

pub type Relations = HashMap<String, Vec<(String, Relation)>>;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub role: Role,
    pub locations: Vec<Location>,
    pub is_murderer: bool
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Character: {}", self.name)?;
        writeln!(f, "  Role: {:?}", self.role)?;
        writeln!(f, "  Murderer: {}", self.is_murderer)?;
        writeln!(f, "  Locations:")?;
        for l in &self.locations {
            writeln!(f, "    - {} ({:?})", l.name, l._type)?;
        }
        Ok(())
    }
}


pub fn role_to_loc(role: Role) -> LocEnum {
    match role {
        Role::Chef => Restaurant,
        Role::Janitor => Hotel,
        Role::Teacher => School,
        Role::Priest => Church,
        Role::Bankteller => Bank,
        Role::Trainer => Gym
    }
}

pub fn loc_to_role(loc: LocEnum) -> Role {
    match loc {
        Restaurant => Chef,
        Hotel => Janitor,
        School => Teacher,
        Church => Priest,
        Bank => Bankteller,
        Gym => Trainer
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Role {
    Chef,
    Janitor,
    Teacher,
    Priest,
    Bankteller,
    Trainer
}

#[derive(AsRefStr, EnumString, Display, Debug, Clone, Serialize, Deserialize, PartialEq)]
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
