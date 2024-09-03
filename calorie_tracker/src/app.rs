use crate::day::Day;
use crate::food::Food;
use crate::storage::{load_days, save_days};
use chrono::{Local, NaiveDate};
use std::error::Error;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub struct App {
    days: Vec<Day>,
    file_path: String,
    current_day_index: usize,
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
        })
    }

    pub fn add_food(&mut self, name: &str, protein: f32, fat: f32, carbs: f32) -> AppResult<()> {
        let food = Food::new(name, protein, fat, carbs);
        self.get_current_day_mut()?.add_food(food);
        self.save()
    }

    pub fn remove_food(&mut self, index: usize) -> AppResult<()> {
        self.get_current_day_mut()?.remove_food(index);
        self.save()
    }

    pub fn reset_day(&mut self) -> AppResult<()> {
        self.get_current_day_mut()?.reset();
        self.save()
    }

    pub fn register_day(&mut self) -> AppResult<()> {
        let new_day = Day::new(Local::now().date_naive());
        self.days.push(new_day);
        self.current_day_index = self.days.len() - 1;
        self.save()
    }

    pub fn search_food(&self, query: &str) -> Vec<&Food> {
        self.days
            .iter()
            .flat_map(|day| day.foods.iter())
            .filter(|food| food.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
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

    pub fn get_week_calories(&self) -> Vec<(NaiveDate, u32)> {
        let current_date = self.get_current_day().unwrap().date;
        let week_start = current_date - chrono::Duration::days(6);

        (0..7)
            .map(|i| {
                let date = week_start + chrono::Duration::days(i);
                let calories = self
                    .days
                    .iter()
                    .find(|day| day.date == date)
                    .map(|day| day.total_calories())
                    .unwrap_or(0);
                (date, calories)
            })
            .collect()
    }

    fn save(&self) -> AppResult<()> {
        save_days(&self.file_path, &self.days)
    }
}

