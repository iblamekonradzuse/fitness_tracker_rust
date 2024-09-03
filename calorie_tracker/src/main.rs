mod app;
mod day;
mod food;
mod storage;

use app::{App, AppResult};
use chrono::NaiveDate;
use food::Food;
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
    println!("Do you want to search for an existing food or enter a new one?");
    println!("1. Search for existing food");
    println!("2. Enter new food");

    let choice: usize = read_usize()?;

    let food = match choice {
        1 => search_and_select_food(app)?,
        2 => enter_new_food()?,
        _ => {
            println!("Invalid choice. Entering new food.");
            enter_new_food()?
        }
    };

    println!("Enter quantity consumed:");
    let quantity: f32 = read_float()?;

    app.add_food(food, quantity)?;
    println!("Food added successfully!");
    Ok(())
}

fn search_and_select_food(app: &App) -> AppResult<Food> {
    println!("Enter food name to search:");
    let query = read_line()?;

    let results = app.search_food(&query);

    if results.is_empty() {
        println!("No matching foods found. Please enter a new food.");
        return enter_new_food();
    }

    println!("Search results:");
    for (i, (food, score)) in results.iter().enumerate() {
        println!("{}. {} (Match score: {})", i + 1, food.name, score);
    }

    println!("Select a food (enter the number) or 0 to enter a new food:");
    let choice: usize = read_usize()?;

    if choice == 0 || choice > results.len() {
        enter_new_food()
    } else {
        Ok(results[choice - 1].0.clone())
    }
}

fn enter_new_food() -> AppResult<Food> {
    println!("Enter food name:");
    let name = read_line()?;

    println!("Enter protein (grams):");
    let protein: f32 = read_float()?;

    println!("Enter fat (grams):");
    let fat: f32 = read_float()?;

    println!("Enter carbs (grams):");
    let carbs: f32 = read_float()?;

    Ok(Food::new(&name, protein, fat, carbs, 1.0))
}

fn read_line() -> AppResult<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
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
    let query = read_line()?;

    let results = app.search_food(&query);
    if results.is_empty() {
        println!("No foods found matching the query.");
    } else {
        println!("Search results:");
        for (food, score) in results {
            println!(
                "- {} ({} calories) [Match score: {}]",
                food.name,
                food.calories(),
                score
            );
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
