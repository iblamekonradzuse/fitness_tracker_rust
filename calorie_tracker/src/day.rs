use crate::app::Workout;
use crate::food::Food;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub foods: Vec<Food>,
    pub workout: Option<Workout>,
}

impl Day {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            foods: Vec::new(),
            workout: None,
        }
    }

    pub fn add_food(&mut self, food: Food, quantity: f64) {
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

    pub fn add_workout(&mut self, workout: Workout) {
        self.workout = Some(workout);
    }

    pub fn total_calories(&self) -> f64 {
        self.foods.iter().map(|food| food.calories()).sum()
    }

    pub fn total_protein(&self) -> f64 {
        self.foods.iter().map(|food| food.protein_content()).sum()
    }

    pub fn reset(&mut self) {
        self.foods.clear();
    }

    pub fn net_calories(&self, bmr: f64) -> f64 {
        let consumed = self.total_calories();
        let burnt = self
            .workout
            .as_ref()
            .map_or(0.0, |w| w.calories_burnt as f64);
        consumed - burnt - bmr
    }
}

