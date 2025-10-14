use chrono::Utc;
use rusqlite::{params, Connection};

use crate::shared::types::Error;

pub fn db_path() -> String {
    std::env::var("DB_PATH").unwrap_or_else(|_| "shaggy.db".to_string())
}

pub fn init_db() -> Result<(), Error> {
    let path = db_path();
    let conn = Connection::open(path)?;

    // Table for logging daily recipes
    conn.execute(
        "CREATE TABLE IF NOT EXISTS daily_recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id TEXT NOT NULL,
            title TEXT NOT NULL,
            sent_at TEXT NOT NULL
        )",
        [],
    )?;

    // Table for users who opted into TTS per guild
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tts_signups (
            user_id INTEGER NOT NULL,
            guild_id INTEGER NOT NULL,
            signed_at TEXT NOT NULL,
            PRIMARY KEY (user_id, guild_id)
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

pub fn tts_signup(user_id: i64, guild_id: i64) -> Result<(), Error> {
    let conn = Connection::open(db_path())?;
    let signed_at = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR REPLACE INTO tts_signups (user_id, guild_id, signed_at) VALUES (?1, ?2, ?3)",
        params![user_id, guild_id, signed_at],
    )?;
    Ok(())
}

pub fn tts_signout(user_id: i64, guild_id: i64) -> Result<(), Error> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "DELETE FROM tts_signups WHERE user_id = ?1 AND guild_id = ?2",
        params![user_id, guild_id],
    )?;
    Ok(())
}

pub fn tts_is_signed(user_id: i64, guild_id: i64) -> Result<bool, Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare("SELECT 1 FROM tts_signups WHERE user_id = ?1 AND guild_id = ?2 LIMIT 1")?;
    Ok(stmt.exists(params![user_id, guild_id])?)
}
