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

    pub fn add_food(&mut self, food: Food) {
        self.foods.push(food);
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

