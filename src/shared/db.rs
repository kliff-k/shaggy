use chrono::Utc;
use rusqlite::{params, Connection};

use crate::shared::types::Error;

pub fn db_path() -> String {
    std::env::var("RECIPE_DB_PATH").unwrap_or_else(|_| "recipes.db".to_string())
}

pub fn init_db() -> Result<(), Error> {
    let path = db_path();
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS daily_recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id TEXT NOT NULL,
            title TEXT NOT NULL,
            sent_at TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn was_recipe_sent(recipe_id: &str) -> Result<bool, Error> {
    let path = db_path();
    let conn = Connection::open(path)?;
    let mut stmt = conn.prepare("SELECT 1 FROM daily_recipes WHERE recipe_id = ?1 LIMIT 1")?;
    let exists = stmt.exists(params![recipe_id])?;
    Ok(exists)
}

pub fn log_recipe_sent(recipe_id: &str, title: &str) -> Result<(), Error> {
    let path = db_path();
    let conn = Connection::open(path)?;
    let sent_at = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO daily_recipes (recipe_id, title, sent_at) VALUES (?1, ?2, ?3)",
        params![recipe_id, title, sent_at],
    )?;
    Ok(())
}
