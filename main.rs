use calorie_tracker::app::{Workout, WorkoutType};
use calorie_tracker::{App, AppResult, Food, Gender};
use chrono::NaiveDate;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() -> AppResult<()> {
    let mut app = App::new("calories.json")?;

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("{}", "🍏🍎 Calorie Tracker 🍎🍏".green().bold());
        println!("{}", "━".repeat(30).cyan());
        println!(
            "{}",
            format!("Day {}", app.current_day())
                .bold()
                .cyan()
                .to_string()
                .center(40)
                .color(colored::Color::Cyan)
        );
        println!("{}", "━".repeat(30).cyan());
        println!();

        let choices = vec![
            "➕ Add food",
            "➖ Remove food",
            "💪 Add workout",
            "🔄 Reset day",
            "📅 Register day",
            "📊 Show current day",
            "🔍 Search food",
            "📆 Change day",
            "📈 Show week calories",
            "👤 Set user info",
            "📏 Calculate BMI",
            "🔥 Calculate BMR",
            "❌ Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => add_food(&mut app)?,
            1 => remove_food(&mut app)?,
            2 => add_workout(&mut app)?,
            3 => reset_day(&mut app)?,
            4 => register_day(&mut app)?,
            5 => show_current_day(&app)?,
            6 => search_food(&app)?,
            7 => change_day(&mut app)?,
            8 => show_week_calories(&app)?,
            9 => set_user_info(&mut app)?,
            10 => calculate_bmi(&app)?,
            11 => calculate_bmr(&app)?,
            12 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn add_food(app: &mut App) -> AppResult<()> {
    loop {
        let choices = vec![
            "🔍 Search for existing food",
            "🆕 Enter new food",
            "⬅️ Go back",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to search for an existing food or enter a new one?")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => {
                if let Some(food) = search_and_select_food(app)? {
                    let quantity: f32 = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter quantity consumed")
                        .interact_text()?;

                    app.add_food(food, quantity)?;
                    println!("\n{}", "✅ Food added successfully!".green());
                    pause()?;
                }
            }
            1 => {
                if let Some(food) = enter_new_food()? {
                    let quantity: f32 = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter quantity consumed")
                        .interact_text()?;

                    app.add_food(food, quantity)?;
                    println!("\n{}", "✅ Food added successfully!".green());
                    pause()?;
                }
            }
            2 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn search_and_select_food(app: &App) -> AppResult<Option<Food>> {
    let query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter food name to search")
        .interact_text()?;

    let results = app.search_food(&query);

    if results.is_empty() {
        println!("\n{}", "❌ No matching foods found.".yellow());
        pause()?;
        return Ok(None);
    }

    println!("\n{}", "🔍 Search results:".cyan());
    let mut choices: Vec<String> = results
        .iter()
        .enumerate()
        .map(|(i, (food, score))| format!("{}. {} (Match score: {})", i + 1, food.name, score))
        .collect();
    choices.push("⬅️ Go back".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a food or go back")
        .default(0)
        .items(&choices)
        .interact()?;

    if selection == choices.len() - 1 {
        Ok(None)
    } else {
        Ok(Some(results[selection].0.clone()))
    }
}

fn enter_new_food() -> AppResult<Option<Food>> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter food name (or leave empty to go back)")
        .allow_empty(true)
        .interact_text()?;

    if name.is_empty() {
        return Ok(None);
    }

    let protein: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter protein (grams)")
        .interact_text()?;

    let fat: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter fat (grams)")
        .interact_text()?;

    let carbs: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter carbs (grams)")
        .interact_text()?;

    Ok(Some(Food::new(&name, protein, fat, carbs, 1.0)))
}

fn remove_food(app: &mut App) -> AppResult<()> {
    loop {
        let day = app.get_current_day()?;
        println!("\n{}", "🍽️ Current foods:".cyan());
        let mut choices: Vec<String> = day
            .foods
            .iter()
            .enumerate()
            .map(|(i, food)| format!("{}. {} ({} calories)", i + 1, food.name, food.calories()))
            .collect();
        choices.push("⬅️ Go back".to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a food to remove or go back")
            .default(0)
            .items(&choices)
            .interact()?;

        if selection == choices.len() - 1 {
            break;
        } else {
            app.remove_food(selection)?;
            println!("\n{}", "✅ Food removed successfully!".green());
            pause()?;
        }
    }
    Ok(())
}

fn add_workout(app: &mut App) -> AppResult<()> {
    let workout_types = vec!["Weight Lifting", "Cardio"];
    let workout_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select workout type")
        .default(0)
        .items(&workout_types)
        .interact()?;

    let workout_type = match workout_type {
        0 => WorkoutType::WeightLifting,
        1 => WorkoutType::Cardio,
        _ => unreachable!(),
    };

    let durations = vec!["30 min", "60 min", "90 min", "120 min", "Other"];
    let duration_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select workout duration")
        .default(0)
        .items(&durations)
        .interact()?;

    let duration = match duration_index {
        0..=3 => (duration_index + 1) * 30,
        4 => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter custom duration (in minutes)")
            .interact_text()?,
        _ => unreachable!(),
    };

    let mut workout = Workout::new(workout_type, duration as u32);

    if workout_type == WorkoutType::Cardio {
        let calories: u32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter calories burnt during cardio")
            .interact_text()?;
        workout.set_cardio_calories(calories);
    }

    app.add_workout(workout)?;
    println!("\n{}", "✅ Workout added successfully!".green());
    pause()?;
    Ok(())
}

fn reset_day(app: &mut App) -> AppResult<()> {
    app.reset_day()?;
    println!("\n{}", "✅ Day reset successfully!".green());
    pause()?;
    Ok(())
}

fn register_day(app: &mut App) -> AppResult<()> {
    app.register_day()?;
    println!("\n{}", "✅ New day registered successfully!".green());
    pause()?;
    Ok(())
}

fn show_current_day(app: &App) -> AppResult<()> {
    let day = app.get_current_day()?;
    println!("\n{}", format!("📅 Current day: {}", day.date).cyan());
    println!(
        "{}",
        format!("🔢 Total calories: {}", day.total_calories()).yellow()
    );
    println!("\n{}", "🍽️ Foods consumed:".cyan());
    for food in &day.foods {
        println!(
            "  • {} ({} calories)",
            food.name.green(),
            food.calories().to_string().yellow()
        );
    }
    pause()?;
    Ok(())
}

fn search_food(app: &App) -> AppResult<()> {
    loop {
        let query: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter search query (or leave empty to go back)")
            .allow_empty(true)
            .interact_text()?;

        if query.is_empty() {
            break;
        }

        let results = app.search_food(&query);
        if results.is_empty() {
            println!("\n{}", "❌ No foods found matching the query.".yellow());
        } else {
            println!("\n{}", "🔍 Search results:".cyan());
            for (food, score) in results {
                println!(
                    "  • {} ({} calories) [Match score: {}]",
                    food.name.green(),
                    food.calories().to_string().yellow(),
                    score.to_string().cyan()
                );
            }
        }
        pause()?;
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
            println!("\n{}", format!("✅ Changed to day: {}", date).green());
        }
        Err(_) => println!(
            "\n{}",
            "❌ Invalid date format. Please use YYYY-MM-DD.".red()
        ),
    }
    pause()?;
    Ok(())
}

fn show_week_calories(app: &App) -> AppResult<()> {
    let week_data = app.get_week_calories_and_workouts();
    let bmr = app.calculate_bmr();

    println!(
        "\n{}",
        "📊 Calories and workouts in the last 7 days:".cyan()
    );
    for &(date, calories, workout) in &week_data {
        let net_calories =
            calories as i32 - workout.map_or(0, |w| w.calories_burnt) as i32 - bmr as i32;
        let net_calories_str = if net_calories > 0 {
            format!("+{}", net_calories).red()
        } else {
            format!("{}", net_calories).green()
        };

        print!(
            "  {} : {} calories",
            date.to_string().green(),
            calories.to_string().yellow()
        );

        if let Some(w) = workout {
            print!(
                " (Workout: {} min {})",
                w.duration.to_string().cyan(),
                match w.workout_type {
                    WorkoutType::WeightLifting => "Weight Lifting",
                    WorkoutType::Cardio => "Cardio",
                }
                .magenta()
            );
        }

        println!(" Net: {} calories", net_calories_str);
    }

    let max_calories = week_data.iter().map(|(_, c, _)| *c).max().unwrap_or(0);
    let scale = 50.0 / max_calories as f32;

    println!("\n{}", "📈 Week Calorie Graph:".cyan());
    for &(date, calories, workout) in &week_data {
        let bar_length = (calories as f32 * scale).round() as usize;
        print!("  {} : ", date.to_string().green());
        print!("{}", "█".repeat(bar_length).yellow());
        print!(" {}", calories.to_string().yellow());

        if let Some(w) = workout {
            print!(" ({})", w.calories_burnt.to_string().red());
        }
        println!();
    }

    pause()?;
    Ok(())
}

fn set_user_info(app: &mut App) -> AppResult<()> {
    let height: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your height (cm)")
        .interact_text()?;

    let weight: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your weight (kg)")
        .interact_text()?;

    let age: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your age")
        .interact_text()?;

    let gender_choices = vec!["Male", "Female"];
    let gender_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your gender")
        .default(0)
        .items(&gender_choices)
        .interact()?;

    let gender = match gender_selection {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => unreachable!(),
    };

    app.set_user_info(height, weight, age, gender);
    println!("\n{}", "✅ User information updated successfully!".green());
    pause()?;
    Ok(())
}

fn calculate_bmi(app: &App) -> AppResult<()> {
    let bmi = app.calculate_bmi();
    println!("\n{}", format!("Your BMI: {:.2}", bmi).cyan());

    let bmi_category = match bmi {
        bmi if bmi < 18.5 => "Underweight",
        bmi if bmi < 25.0 => "Normal weight",
        bmi if bmi < 30.0 => "Overweight",
        _ => "Obese",
    };

    println!("{}", format!("BMI Category: {}", bmi_category).yellow());
    pause()?;
    Ok(())
}

fn calculate_bmr(app: &App) -> AppResult<()> {
    let bmr = app.calculate_bmr();
    println!(
        "\n{}",
        format!("Your Basal Metabolic Rate (BMR): {:.2} calories/day", bmr).cyan()
    );

    println!("\n{}", "Estimated daily calorie needs:".yellow());
    println!(
        "Sedentary (little to no exercise): {:.2} calories",
        bmr * 1.2
    );
    println!(
        "Light exercise (1-3 days/week): {:.2} calories",
        bmr * 1.375
    );
    println!(
        "Moderate exercise (3-5 days/week): {:.2} calories",
        bmr * 1.55
    );
    println!(
        "Heavy exercise (6-7 days/week): {:.2} calories",
        bmr * 1.725
    );
    println!(
        "Very heavy exercise (twice per day): {:.2} calories",
        bmr * 1.9
    );

    pause()?;
    Ok(())
}

fn pause() -> AppResult<()> {
    Input::<String>::new()
        .with_prompt("Press Enter to continue")
        .allow_empty(true)
        .interact_text()?;
    Ok(())
}

trait CenterText {
    fn center(&self, width: usize) -> String;
}

use colored::ColoredString;

impl CenterText for ColoredString {
    fn center(&self, width: usize) -> String {
        self.to_string().center(width)
    }
}

impl CenterText for String {
    fn center(&self, width: usize) -> String {
        if self.len() >= width {
            self.clone()
        } else {
            let left_padding = (width - self.len()) / 2;
            let right_padding = width - self.len() - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                self,
                " ".repeat(right_padding)
            )
        }
    }
}
