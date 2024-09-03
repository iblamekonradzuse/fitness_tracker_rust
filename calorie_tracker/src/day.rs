use crate::food::Food;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub foods: Vec<Food>,
}

impl Day {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            foods: Vec::new(),
        }
    }

    pub fn add_food(&mut self, food: Food, quantity: f32) {
        if let Some(existing_food) = self.foods.iter_mut().find(|f| f.name == food.name) {
            existing_food.quantity += quantity;
        } else {
            let mut new_food = food;
            new_food.quantity = quantity;
            self.foods.push(new_food);
        }
    }

    pub fn remove_food(&mut self, index: usize) {
        if index < self.foods.len() {
            self.foods.remove(index);
        }
    }

    pub fn reset(&mut self) {
        self.foods.clear();
    }

    pub fn total_calories(&self) -> u32 {
        self.foods.iter().map(|food| food.calories()).sum()
    }
}

