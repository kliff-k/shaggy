use anyhow::Context as _;
use poise::ChoiceParameter;
use rand::Rng;

use crate::recipe::models::{MealCategory, MealListResponse, MealsResponse};
use crate::shared::utils::{fetch_from_mealdb, format_meal, mealdb_base_url, special_user_id};
use crate::shared::types::{Context, Error};

/// Get meal recipes.
#[poise::command(slash_command, subcommands("random", "by_category", "by_ingredient"))]
pub async fn recipe(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use a recipe subcommand: `random`, `by-category`, or `by-ingredient`.")
        .await?;
    Ok(())
}

/// Get a random recipe
#[poise::command(slash_command, prefix_command)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    if let Some(special) = special_user_id("ESME_USER_ID") {
        if ctx.author().id == special {
            let ingredient = "mushrooms";
            let url = format!("{}filter.php?i={}", mealdb_base_url(), ingredient);
            let response = fetch_from_mealdb(&url)
                .await
                .context(format!("Failed to fetch meals for ingredient {}", ingredient))?;
            let response_text = response
                .text()
                .await
                .context("Failed to read ingredient meal response text")?;
            if response_text == "{\"meals\":null}" {
                ctx.say(format!(
                    "Couldn't find any recipes with ingredient '{}'.",
                    ingredient
                ))
                .await?;
                return Ok(());
            }

            let meals_response: MealListResponse =
                serde_json::from_str(&response_text).context("Failed to parse ingredient meal JSON")?;

            match meals_response.meals {
                Some(meals) if !meals.is_empty() => {
                    let meal_id = &meals[rand::rng().random_range(0..meals.len())].id;

                    let url = format!("{}lookup.php?i={}", mealdb_base_url(), meal_id);
                    let response = fetch_from_mealdb(&url)
                        .await
                        .context(format!("Failed to fetch meal with id {}", meal_id))?;
                    let meals_response: MealsResponse =
                        response.json().await.context("Failed to parse meal JSON")?;

                    match meals_response.meals {
                        Some(meals) if !meals.is_empty() => {
                            let meal = &meals[0];
                            let reply_content = format_meal(meal, false, false);
                            ctx.send(poise::CreateReply::default().embed(reply_content))
                                .await?;
                        }
                        _ => {
                            ctx.say("Couldn't find a recipe.").await?;
                        }
                    }
                }
                _ => {
                    ctx.say(format!(
                        "Couldn't find any recipes with ingredient '{}'.",
                        ingredient
                    ))
                    .await?;
                }
            }
            return Ok(());
        }
    }

    let url = format!("{}random.php", mealdb_base_url());
    let response = fetch_from_mealdb(&url)
        .await
        .context("Failed to fetch random meal")?;
    let meals_response: MealsResponse =
        response.json().await.context("Failed to parse random meal JSON")?;

    match meals_response.meals {
        Some(meals) if !meals.is_empty() => {
            let meal = &meals[0];
            let reply_content = format_meal(meal, false, false);
            ctx.send(poise::CreateReply::default().embed(reply_content))
                .await?;
        }
        _ => {
            ctx.say("Couldn't find a random recipe.").await?;
        }
    }

    Ok(())
}

/// Get recipes by category
#[poise::command(slash_command, prefix_command, rename = "by-category")]
pub async fn by_category(
    ctx: Context<'_>,
    #[description = "Category to search for"] category: MealCategory,
) -> Result<(), Error> {
    ctx.defer().await?;
    let category_name = category.name();
    let url = format!(
        "{}filter.php?c={}",
        mealdb_base_url(),
        category_name
    );
    let response = fetch_from_mealdb(&url)
        .await
        .context(format!("Failed to fetch meals for category {}", category_name))?;
    let meals_response: MealListResponse =
        response.json().await.context("Failed to parse category meal JSON")?;

    match meals_response.meals {
        Some(meals) if !meals.is_empty() => {
            let meal_id = &meals[rand::rng().random_range(0..meals.len())].id;

            let url = format!("{}lookup.php?i={}", mealdb_base_url(), meal_id);
            let response = fetch_from_mealdb(&url)
                .await
                .context(format!("Failed to fetch meal with id {}", meal_id))?;
            let meals_response: MealsResponse =
                response.json().await.context("Failed to parse meal JSON")?;

            match meals_response.meals {
                Some(meals) if !meals.is_empty() => {
                    let meal = &meals[0];
                    let reply_content = format_meal(meal, false, false);
                    ctx.send(poise::CreateReply::default().embed(reply_content))
                        .await?;
                }
                _ => {
                    ctx.say("Couldn't find a recipe.").await?;
                }
            }
        }
        _ => {
            ctx.say(format!(
                "Couldn't find any recipes in category '{}'.",
                category_name
            ))
            .await?;
        }
    }

    Ok(())
}

/// Get recipes by ingredient
#[poise::command(slash_command, prefix_command, rename = "by-ingredient")]
pub async fn by_ingredient(
    ctx: Context<'_>,
    #[description = "Ingredient to search for"] ingredient: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let url = format!(
        "{}filter.php?i={}",
        mealdb_base_url(),
        ingredient
    );
    let response = fetch_from_mealdb(&url)
        .await
        .context(format!("Failed to fetch meals for ingredient {}", ingredient))?;
    let response_text = response
        .text()
        .await
        .context("Failed to read ingredient meal response text")?;
    if response_text == "{\"meals\":null}" {
        ctx.say(format!(
            "Couldn't find any recipes with ingredient '{}'.",
            ingredient
        ))
        .await?;
        return Ok(());
    }

    let meals_response: MealListResponse =
        serde_json::from_str(&response_text).context("Failed to parse ingredient meal JSON")?;

    match meals_response.meals {
        Some(meals) if !meals.is_empty() => {
            let meal_id = &meals[rand::rng().random_range(0..meals.len())].id;

            let url = format!("{}lookup.php?i={}", mealdb_base_url(), meal_id);
            let response = fetch_from_mealdb(&url)
                .await
                .context(format!("Failed to fetch meal with id {}", meal_id))?;
            let meals_response: MealsResponse =
                response.json().await.context("Failed to parse meal JSON")?;

            match meals_response.meals {
                Some(meals) if !meals.is_empty() => {
                    let meal = &meals[0];
                    let reply_content = format_meal(meal, false, false);
            ctx.send(poise::CreateReply::default().embed(reply_content))
                        .await?;
                }
                _ => {
                    ctx.say("Couldn't find a recipe.").await?;
                }
            }
        }
        _ => {
            ctx.say(format!(
                "Couldn't find any recipes with ingredient '{}'.",
                ingredient
            ))
            .await?;
        }
    }
    Ok(())
}
