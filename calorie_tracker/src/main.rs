mod app;
mod day;
mod food;
mod storage;

use app::{App, AppResult};
use chrono::NaiveDate;
use std::io::{self, Write};

fn main() -> AppResult<()> {
    let mut app = App::new("calories.json")?;

    loop {
        println!("\nCalorie Tracker - Day {}", app.current_day());
        println!("1. Add food");
        println!("2. Remove food");
        println!("3. Reset day");
        println!("4. Register day");
        println!("5. Show current day");
        println!("6. Search food");
        println!("7. Change day");
        println!("8. Show week calories");
        println!("9. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => add_food(&mut app)?,
            "2" => remove_food(&mut app)?,
            "3" => app.reset_day()?,
            "4" => app.register_day()?,
            "5" => show_current_day(&app)?,
            "6" => search_food(&app)?,
            "7" => change_day(&mut app)?,
            "8" => show_week_calories(&app)?,
            "9" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}

fn add_food(app: &mut App) -> AppResult<()> {
    println!("Enter food name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    println!("Enter protein (grams):");
    let protein: f32 = read_float()?;

    println!("Enter fat (grams):");
    let fat: f32 = read_float()?;

    println!("Enter carbs (grams):");
    let carbs: f32 = read_float()?;

    app.add_food(&name.trim(), protein, fat, carbs)?;
    println!("Food added successfully!");
    Ok(())
}

fn remove_food(app: &mut App) -> AppResult<()> {
    let day = app.get_current_day()?;
    println!("Current foods:");
    for (i, food) in day.foods.iter().enumerate() {
        println!("{}. {} ({} calories)", i + 1, food.name, food.calories());
    }

    println!("Enter the number of the food to remove:");
    let index: usize = read_usize()?;

    if index > 0 && index <= day.foods.len() {
        app.remove_food(index - 1)?;
        println!("Food removed successfully!");
    } else {
        println!("Invalid food number.");
    }
    Ok(())
}

fn show_current_day(app: &App) -> AppResult<()> {
    let day = app.get_current_day()?;
    println!("Current day: {}", day.date);
    println!("Total calories: {}", day.total_calories());
    println!("Foods consumed:");
    for food in &day.foods {
        println!("- {} ({} calories)", food.name, food.calories());
    }
    Ok(())
}

fn search_food(app: &App) -> AppResult<()> {
    println!("Enter search query:");
    let mut query = String::new();
    io::stdin().read_line(&mut query)?;

    let results = app.search_food(query.trim());
    if results.is_empty() {
        println!("No foods found matching the query.");
    } else {
        println!("Search results:");
        for food in results {
            println!("- {} ({} calories)", food.name, food.calories());
        }
    }
    Ok(())
}

fn change_day(app: &mut App) -> AppResult<()> {
    println!("Enter date (YYYY-MM-DD):");
    let mut date_str = String::new();
    io::stdin().read_line(&mut date_str)?;

    match NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d") {
        Ok(date) => {
            app.change_day(date)?;
            println!("Changed to day: {}", date);
        }
        Err(_) => println!("Invalid date format. Please use YYYY-MM-DD."),
    }
    Ok(())
}

fn show_week_calories(app: &App) -> AppResult<()> {
    let week_calories = app.get_week_calories();
    println!("Calories consumed in the last 7 days:");
    for &(date, calories) in &week_calories {
        println!("{}: {} calories", date, calories);
    }

    // Create a simple ASCII graph
    let max_calories = week_calories.iter().map(|(_, c)| *c).max().unwrap_or(0);
    let scale = 50.0 / max_calories as f32;

    println!("\nWeek Calorie Graph:");
    for &(date, calories) in &week_calories {
        let bar_length = (calories as f32 * scale).round() as usize;
        print!("{}: ", date);
        io::stdout().write_all(&vec!['#' as u8; bar_length])?;
        println!(" {}", calories);
    }

    Ok(())
}

fn read_float() -> AppResult<f32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

fn read_usize() -> AppResult<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

