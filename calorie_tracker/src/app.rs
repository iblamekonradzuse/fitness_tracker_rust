use crate::api;
use crate::day::Day;
use crate::food::Food;
use crate::storage::{load_days, save_days};
use chrono::{Local, NaiveDate};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::runtime::Runtime;

pub type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct App {
    days: Vec<Day>,
    file_path: String,
    current_day_index: usize,
    matcher: SkimMatcherV2,
    user_height: f32, // in centimeters
    user_weight: f32, // in kilograms
    user_age: u32,
    user_gender: Gender,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkoutType {
    WeightLifting,
    Cardio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workout {
    pub workout_type: WorkoutType,
    pub duration: u32, // in minutes
    pub calories_burnt: u32,
}

impl App {
    pub fn new(file_path: &str) -> AppResult<Self> {
        let mut days = load_days(file_path)?;
        if days.is_empty() {
            days.push(Day::new(Local::now().date_naive()));
        }
        Ok(Self {
            days,
            file_path: file_path.to_string(),
            current_day_index: 0,
            matcher: SkimMatcherV2::default(),
            user_height: 180.0, // Default values
            user_weight: 79.0,
            user_age: 22,
            user_gender: Gender::Male,
        })
    }

    pub fn add_food(&mut self, food: Food, quantity: f64) -> AppResult<()> {
        self.get_current_day_mut()?.add_food(food, quantity);
        self.save()
    }

    pub fn remove_food(&mut self, index: usize) -> AppResult<()> {
        self.get_current_day_mut()?.remove_food(index);
        self.save()
    }

    pub fn search_food(&self, query: &str) -> Vec<(Food, i64)> {
        self.days
            .iter()
            .flat_map(|day| day.foods.iter())
            .filter_map(|food| {
                self.matcher
                    .fuzzy_match(&food.name, query)
                    .map(|score| (food.clone(), score))
            })
            .collect()
    }

    pub fn add_food_manually(&mut self, food: Food) -> AppResult<()> {
        self.get_current_day_mut()?.add_food(food, 1.0);
        self.save()
    }

    pub fn search_and_add_food(&mut self, query: &str) -> AppResult<Vec<Food>> {
        let rt = Runtime::new()?;
        let nutrition_info = rt.block_on(api::search_and_get_nutrition(query));

        match nutrition_info {
            Ok(info) => {
                let mut added_foods = Vec::new();

                for info in info {
                    let food = Food::new(
                        &info.name,
                        info.quantity,
                        &info.unit,
                        info.protein,
                        info.fat,
                        info.carbs,
                        info.calories,
                    );
                    self.add_food(food.clone(), 1.0)?;
                    added_foods.push(food);
                }

                Ok(added_foods)
            }
            Err(e) => {
                println!("API Error: {}. Falling back to manual entry.", e);
                Ok(Vec::new())
            }
        }
    }

    pub fn get_all_foods(&self) -> Vec<&Food> {
        self.days.iter().flat_map(|day| &day.foods).collect()
    }

    pub fn change_day(&mut self, date: NaiveDate) -> AppResult<()> {
        if let Some(index) = self.days.iter().position(|day| day.date == date) {
            self.current_day_index = index;
            Ok(())
        } else {
            Err("Date not found".into())
        }
    }

    pub fn current_day(&self) -> usize {
        self.current_day_index + 1
    }

    pub fn get_current_day(&self) -> AppResult<&Day> {
        self.days
            .get(self.current_day_index)
            .ok_or_else(|| "No days recorded".into())
    }

    pub fn get_current_day_mut(&mut self) -> AppResult<&mut Day> {
        self.days
            .get_mut(self.current_day_index)
            .ok_or_else(|| "No days recorded".into())
    }

    pub fn calculate_recommended_protein(&self, workouts_per_week: u32) -> f32 {
        let activity_factor = match workouts_per_week {
            0..=1 => 0.8,
            2..=3 => 1.0,
            4..=5 => 1.2,
            _ => 1.4,
        };

        self.user_weight * activity_factor
    }

    pub fn get_week_protein_and_calories(&self) -> Vec<(NaiveDate, u32, f32, Option<&Workout>)> {
        let current_date = self.get_current_day().unwrap().date;
        let week_start = current_date - chrono::Duration::days(6);

        (0..7)
            .map(|i| {
                let date = week_start + chrono::Duration::days(i);
                let day = self.days.iter().find(|day| day.date == date);
                let calories = day.map(|d| d.total_calories()).unwrap_or(0.0);
                let protein = day.map(|d| d.total_protein()).unwrap_or(0.0);
                let workout = day.and_then(|d| d.workout.as_ref());
                (date, calories as u32, protein as f32, workout)
            })
            .collect()
    }

    fn save(&self) -> AppResult<()> {
        save_days(&self.file_path, &self.days)
    }
    pub fn reset_day(&mut self) -> AppResult<()> {
        self.get_current_day_mut()?.reset();
        self.save()
    }
    pub fn register_day(&mut self) -> AppResult<()> {
        let current_date = self.get_current_day()?.date;
        let new_date = current_date.succ_opt().ok_or("Failed to get next date")?;
        if !self.days.iter().any(|day| day.date == new_date) {
            self.days.push(Day::new(new_date));
            self.current_day_index = self.days.len() - 1;
            self.save()?;
        }
        Ok(())
    }
    pub fn set_user_info(&mut self, height: f32, weight: f32, age: u32, gender: Gender) {
        self.user_height = height;
        self.user_weight = weight;
        self.user_age = age;
        self.user_gender = gender;
    }
    pub fn add_workout(&mut self, workout: Workout) -> AppResult<()> {
        self.get_current_day_mut()?.add_workout(workout);
        self.save()
    }

    pub fn get_week_calories_and_workouts(&self) -> Vec<(NaiveDate, f64, Option<&Workout>)> {
        let current_date = self.get_current_day().unwrap().date;
        let week_start = current_date - chrono::Duration::days(6);

        (0..7)
            .map(|i| {
                let date = week_start + chrono::Duration::days(i);
                let day = self.days.iter().find(|day| day.date == date);
                let calories = day.map(|d| d.total_calories()).unwrap_or(0.0);
                let workout = day.and_then(|d| d.workout.as_ref());
                (date, calories, workout)
            })
            .collect()
    }

    pub fn calculate_bmi(&self) -> f32 {
        let height_in_meters = self.user_height / 100.0;
        self.user_weight / (height_in_meters * height_in_meters)
    }

    pub fn calculate_bmr(&self) -> f32 {
        match self.user_gender {
            Gender::Male => {
                88.362 + (13.397 * self.user_weight) + (4.799 * self.user_height)
                    - (5.677 * self.user_age as f32)
            }
            Gender::Female => {
                447.593 + (9.247 * self.user_weight) + (3.098 * self.user_height)
                    - (4.330 * self.user_age as f32)
            }
        }
    }
}

impl Workout {
    pub fn new(workout_type: WorkoutType, duration: u32) -> Self {
        let calories_burnt = match workout_type {
            WorkoutType::WeightLifting => {
                // Estimate calories burnt for weight lifting
                match duration {
                    30 => 120,
                    60 => 220,
                    90 => 330,
                    120 => 440,
                    _ => (duration as f32 * 3.67).round() as u32, // Approximate for other durations
                }
            }
            WorkoutType::Cardio => 0, // This will be set by the user for cardio
        };

        Self {
            workout_type,
            duration,
            calories_burnt,
        }
    }

    pub fn set_cardio_calories(&mut self, calories: u32) {
        if self.workout_type == WorkoutType::Cardio {
            self.calories_burnt = calories;
        }
    }
}
