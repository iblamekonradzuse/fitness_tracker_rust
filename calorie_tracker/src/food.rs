use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub protein: f64,
    pub fat: f64,
    pub carbs: f64,
    pub calories: f64, // Keep this as f64 to maintain precision
}

impl Food {
    pub fn new(
        name: &str,
        quantity: f64,
        unit: &str,
        protein: f64,
        fat: f64,
        carbs: f64,
        calories: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            quantity,
            unit: unit.to_string(),
            protein,
            fat,
            carbs,
            calories,
        }
    }

    pub fn calories(&self) -> f64 {
        self.calories
    }

    pub fn protein_content(&self) -> f64 {
        self.protein
    }
}

