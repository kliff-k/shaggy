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

    // Table for reminders (medicine/food)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS reminders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            guild_id INTEGER,
            kind TEXT NOT NULL,
            time TEXT NOT NULL,
            note TEXT,
            private INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            timezone TEXT NOT NULL DEFAULT 'UTC'
        )",
        [],
    )?;

    // Lightweight migration: ensure timezone column exists for older DBs
    {
        let mut stmt = conn.prepare("PRAGMA table_info(reminders)")?;
        let mut has_tz = false;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let name: String = row.get(1)?; // column name
            if name.eq_ignore_ascii_case("timezone") {
                has_tz = true;
                break;
            }
        }
        if !has_tz {
            let _ = conn.execute(
                "ALTER TABLE reminders ADD COLUMN timezone TEXT NOT NULL DEFAULT 'UTC'",
                [],
            );
        }
    }

    // Table for moderation warnings
    conn.execute(
        "CREATE TABLE IF NOT EXISTS warnings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guild_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            moderator_id INTEGER NOT NULL,
            reason TEXT NOT NULL,
            created_at TEXT NOT NULL
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

pub fn add_reminder(user_id: i64, guild_id: Option<i64>, kind: &str, time: &str, note: Option<&str>, private: bool, timezone: &str) -> Result<(), Error> {
    let conn = Connection::open(db_path())?;
    let created_at = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO reminders (user_id, guild_id, kind, time, note, private, created_at, timezone) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![user_id, guild_id, kind, time, note, private, created_at, timezone],
    )?;
    Ok(())
}

pub fn get_user_reminders_in_guild(user_id: i64, guild_id: i64) -> Result<Vec<ReminderRow>, Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id, user_id, guild_id, kind, time, note, IFNULL(private, 0) as private, timezone FROM reminders WHERE user_id = ?1 AND guild_id = ?2 ORDER BY time ASC, id ASC",
    )?;
    let rows = stmt.query_map(params![user_id, guild_id], |row| {
        let private_val: i64 = row.get(6)?;
        Ok(ReminderRow {
            id: row.get(0)?,
            user_id: row.get(1)?,
            guild_id: row.get(2)?,
            kind: row.get(3)?,
            time: row.get(4)?,
            note: row.get(5)?,
            private: private_val != 0,
            timezone: row.get(7)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn delete_reminder_owned(id: i64, user_id: i64, guild_id: i64) -> Result<bool, Error> {
    let conn = Connection::open(db_path())?;
    let affected = conn.execute(
        "DELETE FROM reminders WHERE id = ?1 AND user_id = ?2 AND guild_id = ?3",
        params![id, user_id, guild_id],
    )?;
    Ok(affected > 0)
}

pub fn get_distinct_timezones() -> Result<Vec<String>, Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare("SELECT DISTINCT timezone FROM reminders")?;
    let rows = stmt.query_map([], |row| {
        let tz: String = row.get(0)?;
        Ok(tz)
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn log_warning(guild_id: i64, user_id: i64, moderator_id: i64, reason: &str) -> Result<(), Error> {
    let conn = Connection::open(db_path())?;
    let created_at = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO warnings (guild_id, user_id, moderator_id, reason, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![guild_id, user_id, moderator_id, reason, created_at],
    )?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct ReminderRow {
    pub id: i64,
    pub user_id: i64,
    pub guild_id: Option<i64>,
    pub kind: String,
    pub time: String,
    pub note: Option<String>,
    pub private: bool,
    pub timezone: String,
}

#[derive(Debug, Clone)]
pub struct WarningRow {
    pub id: i64,
    pub guild_id: i64,
    pub user_id: i64,
    pub moderator_id: i64,
    pub reason: String,
    pub created_at: String,
}

pub fn get_warnings_for_user(guild_id: i64, user_id: i64) -> Result<Vec<WarningRow>, Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id, guild_id, user_id, moderator_id, reason, created_at FROM warnings WHERE guild_id = ?1 AND user_id = ?2 ORDER BY created_at ASC",
    )?;
    let rows = stmt.query_map(params![guild_id, user_id], |row| {
        Ok(WarningRow {
            id: row.get(0)?,
            guild_id: row.get(1)?,
            user_id: row.get(2)?,
            moderator_id: row.get(3)?,
            reason: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn get_reminders_by_time_tz(time: &str, timezone: &str) -> Result<Vec<ReminderRow>, Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id, user_id, guild_id, kind, time, note, IFNULL(private, 0) as private, timezone FROM reminders WHERE time = ?1 AND timezone = ?2",
    )?;

    let rows = stmt.query_map(params![time, timezone], |row| {
        let private_val: i64 = row.get(6)?;
        Ok(ReminderRow {
            id: row.get(0)?,
            user_id: row.get(1)?,
            guild_id: row.get(2)?,
            kind: row.get(3)?,
            time: row.get(4)?,
            note: row.get(5)?,
            private: private_val != 0,
            timezone: row.get(7)?,
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
