use poise::serenity_prelude as serenity;
use crate::recipe::models::{Meal, MealsResponse};
use crate::shared::types::Error;

pub async fn fetch_from_mealdb(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get(url).await?.error_for_status()
}

pub fn mealdb_base_url() -> String {
    let mut base = std::env::var("MEALDB_BASE_URL")
        .unwrap_or_else(|_| "https://www.themealdb.com/api/json/v1/1/".to_string());
    if !base.ends_with('/') {
        base.push('/');
    }
    base
}

pub fn format_meal(meal: &Meal, daily: bool, repeat: bool) -> serenity::CreateEmbed {
    let ingredients = meal.get_ingredients();
    let ingredients_list = ingredients
        .iter()
        .map(|(ing, mea)| format!("- {} ({})", ing, mea))
        .collect::<Vec<_>>()
        .join("\n");

    let daily_str = if daily { "Daily recipe: " } else { "" };
    let repeat_str = if repeat { " (Repeat)" } else { "" };

    serenity::CreateEmbed::new()
        .title(format!("{}{}{}", daily_str, &meal.name, repeat_str))
        .description(format!(
            "**Ingredients:**\n{}\n\n**Instructions:**\n{}",
            ingredients_list, &meal.instructions
        ))
        .color(0x00FF00)
        .thumbnail(meal.thumbnail.clone().unwrap_or_default())
}

pub async fn get_random_meal(_http_client: &reqwest::Client) -> Result<Option<Meal>, Error> {
    let list_url = format!("{}random.php", mealdb_base_url());
    let response = fetch_from_mealdb(&list_url).await?;
    let meal_resp: MealsResponse = response.json().await?;

    if let Some(meals) = meal_resp.meals {
        Ok(meals.into_iter().next())
    } else {
        Ok(None)
    }
}

pub async fn get_and_format_random_recipe(_http_client: &reqwest::Client) -> Result<Option<serenity::CreateEmbed>, Error> {
    if let Some(meal) = get_random_meal(_http_client).await? {
        Ok(Some(format_meal(&meal, true, false)))
    } else {
        Ok(None)
    }
}
