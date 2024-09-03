use crate::day::Day;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn load_days(file_path: &str) -> Result<Vec<Day>, Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let days: Vec<Day> = serde_json::from_str(&contents)?;
    Ok(days)
}

pub fn save_days(file_path: &str, days: &[Day]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(days)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
