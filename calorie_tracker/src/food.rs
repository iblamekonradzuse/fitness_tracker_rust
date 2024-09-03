use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Food {
    pub name: String,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
}

impl Food {
    pub fn new(name: &str, protein: f32, fat: f32, carbs: f32) -> Self {
        Self {
            name: name.to_string(),
            protein,
            fat,
            carbs,
        }
    }

    pub fn calories(&self) -> u32 {
        ((self.protein * 4.0) + (self.fat * 9.0) + (self.carbs * 4.0)).round() as u32
    }
}

