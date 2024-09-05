use crate::day::Day;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn load_days(file_path: &str) -> Result<Vec<Day>, Box<dyn Error + Send + Sync>> {
    let path = Path::new(file_path);
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let days: Vec<Day> = serde_json::from_str(&contents)?;
        Ok(days)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_days(file_path: &str, days: &[Day]) -> Result<(), Box<dyn Error + Send + Sync>> {
    let serialized = serde_json::to_string(days)?;
    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

