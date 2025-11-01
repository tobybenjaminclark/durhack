use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use crate::types::Map;

/// Write a Map to a JSON file
pub fn write_map_to_file(map: &Map, path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, map)?;
    Ok(())
}

/// Read a Map from a JSON file
pub fn read_map_from_file(path: &str) -> io::Result<Map> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}