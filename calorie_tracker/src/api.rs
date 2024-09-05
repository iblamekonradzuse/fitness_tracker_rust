use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct RequestBody<'a> {
    query: &'a str,
    timezone: &'a str,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    foods: Vec<FoodItem>,
}

#[derive(Deserialize, Debug)]
struct FoodItem {
    food_name: String,
    serving_qty: f64,
    serving_unit: String,
    nf_calories: f64,
    nf_total_fat: f64,
    nf_protein: f64,
    nf_total_carbohydrate: Option<f64>,
}

#[derive(Debug)]
pub struct NutritionInfo {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub calories: f64,
    pub protein: f64,
    pub fat: f64,
    pub carbs: f64,
}

pub async fn search_and_get_nutrition(
    query: &str,
) -> Result<Vec<NutritionInfo>, Box<dyn Error + Send + Sync>> {
    let app_id = "4954cf1b";
    let api_key = "0b19a47f300a9bfa16649d584df7ee5d";
    let url = "https://trackapi.nutritionix.com/v2/natural/nutrients";

    let client = Client::new();
    let request_body = RequestBody {
        query,
        timezone: "US/Eastern",
    };

    let response = client
        .post(url)
        .header("x-app-id", app_id)
        .header("x-app-key", api_key)
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;

        let api_response: ApiResponse = serde_json::from_str(&response_text)?;

        let nutrition_info = api_response
            .foods
            .into_iter()
            .map(|food| {
                let carbs = food.nf_total_carbohydrate.unwrap_or_else(|| {
                    (food.nf_calories - (food.nf_protein * 4.0) - (food.nf_total_fat * 9.0)) / 4.0
                });

                NutritionInfo {
                    name: food.food_name,
                    quantity: food.serving_qty,
                    unit: food.serving_unit,
                    calories: food.nf_calories,
                    protein: food.nf_protein,
                    fat: food.nf_total_fat,
                    carbs: (carbs * 10.0).round() / 10.0,
                }
            })
            .collect();

        Ok(nutrition_info)
    } else {
        Err(format!("API request failed: {}", response.status()).into())
    }
}
