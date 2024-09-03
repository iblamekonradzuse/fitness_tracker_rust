use calorie_tracker::{App, AppResult, Food};
use chrono::NaiveDate;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() -> AppResult<()> {
    let mut app = App::new("calories.json")?;

    loop {
        println!("\n{}", "🍏 Calorie Tracker 🍎".green().bold());
        println!("{}", format!("Day {}", app.current_day()).cyan());

        let choices = vec![
            "Add food",
            "Remove food",
            "Reset day",
            "Register day",
            "Show current day",
            "Search food",
            "Change day",
            "Show week calories",
            "Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => add_food(&mut app)?,
            1 => remove_food(&mut app)?,
            2 => reset_day(&mut app)?,
            3 => register_day(&mut app)?,
            4 => show_current_day(&app)?,
            5 => search_food(&app)?,
            6 => change_day(&mut app)?,
            7 => show_week_calories(&app)?,
            8 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn add_food(app: &mut App) -> AppResult<()> {
    let choices = vec!["Search for existing food", "Enter new food"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to search for an existing food or enter a new one?")
        .default(0)
        .items(&choices)
        .interact()?;

    let food = match selection {
        0 => search_and_select_food(app)?,
        1 => enter_new_food()?,
        _ => unreachable!(),
    };

    let quantity: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter quantity consumed")
        .interact_text()?;

    app.add_food(food, quantity)?;
    println!("{}", "Food added successfully!".green());
    Ok(())
}

fn search_and_select_food(app: &App) -> AppResult<Food> {
    let query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter food name to search")
        .interact_text()?;

    let results = app.search_food(&query);

    if results.is_empty() {
        println!(
            "{}",
            "No matching foods found. Please enter a new food.".yellow()
        );
        return enter_new_food();
    }

    println!("{}", "Search results:".cyan());
    let choices: Vec<String> = results
        .iter()
        .enumerate()
        .map(|(i, (food, score))| format!("{}. {} (Match score: {})", i + 1, food.name, score))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a food or choose 'Enter new food'")
        .default(0)
        .items(&choices)
        .item("Enter new food")
        .interact()?;

    if selection == choices.len() {
        enter_new_food()
    } else {
        Ok(results[selection].0.clone())
    }
}

fn enter_new_food() -> AppResult<Food> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter food name")
        .interact_text()?;

    let protein: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter protein (grams)")
        .interact_text()?;

    let fat: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter fat (grams)")
        .interact_text()?;

    let carbs: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter carbs (grams)")
        .interact_text()?;

    Ok(Food::new(&name, protein, fat, carbs, 1.0))
}

fn remove_food(app: &mut App) -> AppResult<()> {
    let day = app.get_current_day()?;
    println!("{}", "Current foods:".cyan());
    let choices: Vec<String> = day
        .foods
        .iter()
        .enumerate()
        .map(|(i, food)| format!("{}. {} ({} calories)", i + 1, food.name, food.calories()))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a food to remove")
        .default(0)
        .items(&choices)
        .interact()?;

    app.remove_food(selection)?;
    println!("{}", "Food removed successfully!".green());
    Ok(())
}

fn reset_day(app: &mut App) -> AppResult<()> {
    app.reset_day()?;
    println!("{}", "Day reset successfully!".green());
    Ok(())
}

fn register_day(app: &mut App) -> AppResult<()> {
    app.register_day()?;
    println!("{}", "New day registered successfully!".green());
    Ok(())
}

fn show_current_day(app: &App) -> AppResult<()> {
    let day = app.get_current_day()?;
    println!("{}", format!("Current day: {}", day.date).cyan());
    println!(
        "{}",
        format!("Total calories: {}", day.total_calories()).yellow()
    );
    println!("{}", "Foods consumed:".cyan());
    for food in &day.foods {
        println!(
            "- {} ({} calories)",
            food.name.green(),
            food.calories().to_string().yellow()
        );
    }
    Ok(())
}

fn search_food(app: &App) -> AppResult<()> {
    let query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter search query")
        .interact_text()?;

    let results = app.search_food(&query);
    if results.is_empty() {
        println!("{}", "No foods found matching the query.".yellow());
    } else {
        println!("{}", "Search results:".cyan());
        for (food, score) in results {
            println!(
                "- {} ({} calories) [Match score: {}]",
                food.name.green(),
                food.calories().to_string().yellow(),
                score.to_string().cyan()
            );
        }
    }
    Ok(())
}

fn change_day(app: &mut App) -> AppResult<()> {
    let date_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter date (YYYY-MM-DD)")
        .interact_text()?;

    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => {
            app.change_day(date)?;
            println!("{}", format!("Changed to day: {}", date).green());
        }
        Err(_) => println!("{}", "Invalid date format. Please use YYYY-MM-DD.".red()),
    }
    Ok(())
}

fn show_week_calories(app: &App) -> AppResult<()> {
    let week_calories = app.get_week_calories();
    println!("{}", "Calories consumed in the last 7 days:".cyan());
    for &(date, calories) in &week_calories {
        println!(
            "{}: {} calories",
            date.to_string().green(),
            calories.to_string().yellow()
        );
    }

    let max_calories = week_calories.iter().map(|(_, c)| *c).max().unwrap_or(0);
    let scale = 50.0 / max_calories as f32;

    println!("\n{}", "Week Calorie Graph:".cyan());
    for &(date, calories) in &week_calories {
        let bar_length = (calories as f32 * scale).round() as usize;
        print!("{}: ", date.to_string().green());
        print!("{}", "█".repeat(bar_length).yellow());
        println!(" {}", calories.to_string().yellow());
    }

    Ok(())
}



