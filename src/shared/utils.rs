use poise::serenity_prelude::UserId;

pub use crate::recipe::utils::{fetch_from_mealdb, format_meal, get_and_format_random_recipe, mealdb_base_url};

pub fn special_user_id(user: &str) -> Option<UserId> {
    match std::env::var(user) {
        Ok(val) => match val.trim().parse::<u64>() {
            Ok(id) => Some(UserId::new(id)),
            Err(_) => None,
        },
        Err(_) => None,
    }
}