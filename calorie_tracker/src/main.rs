use calorie_tracker::app::{Workout, WorkoutType};
use calorie_tracker::{App, AppResult, Food, Gender};
use chrono::NaiveDate;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::cmp::min;

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
            "📅 Day Management",
            "🍽️ Food Tracking",
            "💪 Workout Management",
            "📊 Statistics and Reports",
            "👤 User Settings",
            "❌ Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => day_management_menu(&mut app)?,
            1 => food_tracking_menu(&mut app)?,
            2 => workout_management_menu(&mut app)?,
            3 => statistics_menu(&app)?,
            4 => user_settings_menu(&mut app)?,
            5 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn day_management_menu(app: &mut App) -> AppResult<()> {
    loop {
        let choices = vec![
            "📅 Change day",
            "🔄 Reset day",
            "📆 Register new day",
            "📊 Show current day",
            "⬅️ Back to main menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Day Management")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => change_day(app)?,
            1 => reset_day(app)?,
            2 => register_day(app)?,
            3 => show_current_day(app)?,
            4 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn food_tracking_menu(app: &mut App) -> AppResult<()> {
    loop {
        let choices = vec![
            "➕ Add food",
            "➖ Remove food",
            "🔍 Search food",
            "⬅️ Back to main menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Food Tracking")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => add_food(app)?,
            1 => remove_food(app)?,
            2 => search_food(app)?,
            3 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn workout_management_menu(app: &mut App) -> AppResult<()> {
    loop {
        let choices = vec!["💪 Add workout", "📊 View workouts", "⬅️ Back to main menu"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Workout Management")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => add_workout(app)?,
            1 => view_workouts(app)?,
            2 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn statistics_menu(app: &App) -> AppResult<()> {
    loop {
        let choices = vec![
            "📈 Show week calories",
            "📏 Calculate BMI",
            "🔥 Calculate BMR",
            "⬅️ Back to main menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Statistics and Reports")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => show_week_calories(app)?,
            1 => calculate_bmi(app)?,
            2 => calculate_bmr(app)?,
            3 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn user_settings_menu(app: &mut App) -> AppResult<()> {
    loop {
        let choices = vec!["👤 Set user info", "⬅️ Back to main menu"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("User Settings")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => set_user_info(app)?,
            1 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn add_food(app: &mut App) -> AppResult<()> {
    let choices = vec![
        "✏️ Add food manually",
        "🌐 Search and add food from API",
        "⬅️ Back to main menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&choices)
        .interact()?;

    match selection {
        0 => add_food_manually(app),
        1 => search_and_add_food(app),
        2 => Ok(()),
        _ => unreachable!(),
    }
}

fn add_food_manually(app: &mut App) -> AppResult<()> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter food name")
        .interact_text()?;

    let quantity: f64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter quantity (e.g., 1, 2, 0.5)")
        .interact_text()?;

    let protein: f64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter protein content (grams)")
        .interact_text()?;

    let fat: f64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter fat content (grams)")
        .interact_text()?;

    let carbs: f64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter carbohydrate content (grams)")
        .interact_text()?;

    // Calculate calories
    let calories = (protein * 4.0) + (fat * 9.0) + (carbs * 4.0);

    let food = Food::new(&name, quantity, "serving", protein, fat, carbs, calories);
    app.add_food_manually(food)?;

    println!("\n{}", "✅ Food added successfully!".green());
    pause()?;
    Ok(())
}

fn search_and_add_food(app: &mut App) -> AppResult<()> {
    let query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter food and quantity (e.g., '2 apples, 200 grams of chicken') ")
        .interact_text()?;

    match app.search_and_add_food(&query) {
        Ok(added_foods) => {
            if added_foods.is_empty() {
                println!("\n{}", "❌ No foods found matching the query.".yellow());
                println!("{}", "You can try adding the food manually.".yellow());
            } else {
                println!("\n{}", "✅ Foods added successfully:".green());
                for food in added_foods {
                    println!(
                        "  • {} {} {}",
                        food.quantity.to_string().yellow(),
                        food.unit.cyan(),
                        food.name.green()
                    );
                    println!(
                        "    Calories: {}, Protein: {}g, Fat: {}g, Carbs: {}g",
                        food.calories().to_string().red(),
                        food.protein_content().to_string().blue(),
                        food.fat.to_string().magenta(),
                        food.carbs.to_string().yellow()
                    );
                }
            }
        }
        Err(e) => {
            println!("\n{}", format!("❌ Error adding food: {}", e).red());
            println!("{}", "You can try adding the food manually.".yellow());
        }
    }

    pause()?;
    Ok(())
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

fn view_workouts(app: &App) -> AppResult<()> {
    let day = app.get_current_day()?;

    if let Some(workout) = &day.workout {
        println!("\n{}", "💪 Today's Workout:".cyan());
        println!(
            "  Type: {}",
            match workout.workout_type {
                WorkoutType::WeightLifting => "Weight Lifting",
                WorkoutType::Cardio => "Cardio",
            }
            .green()
        );
        println!(
            "  Duration: {} minutes",
            workout.duration.to_string().yellow()
        );
        println!(
            "  Calories Burnt: {}",
            workout.calories_burnt.to_string().red()
        );
    } else {
        println!("\n{}", "No workout recorded for today.".yellow());
    }

    pause()?;
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
        format!("🔢 Total calories: {:.0}", day.total_calories()).yellow()
    );
    println!("\n{}", "🍽️ Foods consumed:".cyan());
    for food in &day.foods {
        println!(
            "  • {} ({:.0} calories)",
            food.name.green(),
            food.calories()
        );
    }
    pause()?;
    Ok(())
}

fn search_food(app: &App) -> AppResult<()> {
    loop {
        let choices = vec![
            "🍽️ Browse all foods",
            "🔍 Search for a specific food",
            "⬅️ Back to main menu",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&choices)
            .interact()?;

        match selection {
            0 => browse_foods(app)?,
            1 => search_specific_food(app)?,
            2 => break,
            _ => unreachable!(),
        }
    }
    Ok(())
}

const ITEMS_PER_PAGE: usize = 10;

fn browse_foods(app: &App) -> AppResult<()> {
    let all_foods: Vec<&Food> = app.get_all_foods();

    if all_foods.is_empty() {
        println!("\n{}", "No foods have been added yet.".yellow());
        pause()?;
        return Ok(());
    }

    let total_pages = (all_foods.len() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
    let mut current_page = 1;

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("{}", "🍽️ All added foods".green().bold());
        println!("{}", "━".repeat(30).cyan());

        let start_index = (current_page - 1) * ITEMS_PER_PAGE;
        let end_index = min(start_index + ITEMS_PER_PAGE, all_foods.len());

        let mut choices: Vec<String> = all_foods[start_index..end_index]
            .iter()
            .enumerate()
            .map(|(index, food)| {
                format!(
                    "{:<3} {} ({:.0} calories, {:.1}g protein, {:.1}g fat, {:.1}g carbs)",
                    index + start_index + 1,
                    food.name.green(),
                    food.calories(),
                    food.protein,
                    food.fat,
                    food.carbs
                )
            })
            .collect();

        println!(
            "\n{}",
            format!("Page {} of {}", current_page, total_pages).cyan()
        );
        println!("{}", "━".repeat(30).cyan());

        choices.push("⬅️ Previous page".to_string());
        choices.push("➡️ Next page".to_string());
        choices.push("⬅️ Back to menu".to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a food or an option")
            .default(0)
            .items(&choices)
            .interact()?;

        if selection < end_index - start_index {
            // A food was selected
            let selected_food = all_foods[start_index + selection];
            println!("\n{}", "Selected food:".cyan());
            println!(
                "  {} ({:.0} calories, {:.1}g protein, {:.1}g fat, {:.1}g carbs)",
                selected_food.name.green(),
                selected_food.calories(),
                selected_food.protein,
                selected_food.fat,
                selected_food.carbs
            );
            pause()?;
        } else {
            // An option was selected
            match selection - (end_index - start_index) {
                0 if current_page > 1 => current_page -= 1,
                1 if current_page < total_pages => current_page += 1,
                2 => break,
                _ => {}
            }
        }
    }

    Ok(())
}

fn search_specific_food(app: &App) -> AppResult<()> {
    let query: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter search query")
        .interact_text()?;

    let results = app.search_food(&query);
    if results.is_empty() {
        println!("\n{}", "❌ No foods found matching the query.".yellow());
    } else {
        println!("\n{}", "🔍 Search results:".cyan());
        for (food, score) in results {
            println!(
                "  • {} ({:.0} calories, {:.1}g protein, {:.1}g fat, {:.1}g carbs) [Match score: {}]",
                food.name.green(),
                food.calories(),
                food.protein,
                food.fat,
                food.carbs,
                score.to_string().cyan()
            );
        }
    }
    pause()?;
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
    let workouts_per_week: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("How many times do you work out per week?")
        .interact_text()?;

    let recommended_protein = app.calculate_recommended_protein(workouts_per_week);
    let week_data = app.get_week_protein_and_calories();
    let bmr = app.calculate_bmr();

    println!(
        "\n{}",
        "📊 Calories, protein, and workouts in the last 7 days:".cyan()
    );
    for &(date, calories, protein, workout) in &week_data {
        let net_calories =
            calories as i32 - workout.map_or(0, |w| w.calories_burnt) as i32 - bmr as i32;
        let net_calories_str = if net_calories > 0 {
            format!("+{}", net_calories).red()
        } else {
            format!("{}", net_calories).green()
        };

        print!(
            "  {} : {} calories, {:.1}g protein",
            date.to_string().green(),
            calories,
            protein
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

    let max_calories = week_data.iter().map(|(_, c, _, _)| *c).max().unwrap_or(0);
    let max_protein = week_data
        .iter()
        .map(|(_, _, p, _)| *p)
        .fold(0.0f32, |a: f32, b| a.max(b));

    let scale_calories = 50.0 / max_calories as f32;
    let scale_protein = 50.0 / max_protein;

    println!("\n{}", "📈 Week Calorie and Protein Graph:".cyan());
    for &(date, calories, protein, _) in &week_data {
        let cal_bar_length = (calories as f32 * scale_calories).round() as usize;
        let prot_bar_length = (protein * scale_protein).round() as usize;

        println!("  {} :", date.to_string().green());
        print!("    Calories: ");
        print!("{}", "█".repeat(cal_bar_length).yellow());
        println!(" {}", calories.to_string().yellow());

        print!("    Protein:  ");
        print!("{}", "█".repeat(prot_bar_length).cyan());
        println!(" {:.1}g", protein);
    }

    println!(
        "\n{}",
        format!(
            "Recommended daily protein intake: {:.1}g",
            recommended_protein
        )
        .magenta()
    );

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
