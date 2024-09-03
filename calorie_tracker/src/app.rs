use crate::day::Day;
use crate::food::Food;
use crate::storage::{load_days, save_days};
use chrono::{Local, NaiveDate};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::error::Error;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub struct App {
    days: Vec<Day>,
    file_path: String,
    current_day_index: usize,
    matcher: SkimMatcherV2,
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
        })
    }

    pub fn add_food(&mut self, food: Food, quantity: f32) -> AppResult<()> {
        self.get_current_day_mut()?.add_food(food, quantity);
        self.save()
    }

    pub fn remove_food(&mut self, index: usize) -> AppResult<()> {
        self.get_current_day_mut()?.remove_food(index);
        self.save()
    }

    pub fn search_food(&self, query: &str) -> Vec<(&Food, i64)> {
        self.days
            .iter()
            .flat_map(|day| day.foods.iter())
            .filter_map(|food| {
                self.matcher
                    .fuzzy_match(&food.name, query)
                    .map(|score| (food, score))
            })
            .collect()
    }

    pub fn get_all_foods(&self) -> Vec<&Food> {
        self.days.iter().flat_map(|day| day.foods.iter()).collect()
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
    pub fn reset_day(&mut self) -> AppResult<()> {
        self.get_current_day_mut()?.reset();
        self.save()
    }

    pub fn register_day(&mut self) -> AppResult<()> {
        let new_date = self.get_current_day()?.date.succ();
        if !self.days.iter().any(|day| day.date == new_date) {
            self.days.push(Day::new(new_date));
            self.current_day_index = self.days.len() - 1;
            self.save()?;
        }
        Ok(())
    }
}
