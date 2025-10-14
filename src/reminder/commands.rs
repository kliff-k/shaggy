use chrono::NaiveTime;
use poise::serenity_prelude as serenity;

use crate::shared::db::add_reminder;
use crate::shared::types::{Context, Error};

/// Reminder-related commands
#[poise::command(slash_command, subcommands("register"))]
pub async fn remind(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Register a reminder for medicine or food at a specific time (HH:MM, 24h)
#[poise::command(slash_command, guild_only)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Type of reminder"] kind: String,
    #[description = "Time in HH:MM (24h) e.g. 08:30"] time: String,
    #[description = "Optional note (e.g., medicine name)"] note: Option<String>,
    #[description = "Send this reminder privately via DM (instead of the public channel)?"] private: bool,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        None => {
            ctx.say("This command can only be used in a server.").await?;
            return Ok(());
        }
    };

    let kind_lc = kind.trim().to_lowercase();
    if kind_lc != "medicine" && kind_lc != "food" {
        ctx.send(
            poise::CreateReply::default()
                .content("Invalid kind. Please choose either 'medicine' or 'food'.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    // Validate time format
    if NaiveTime::parse_from_str(time.trim(), "%H:%M").is_err() {
        ctx.send(
            poise::CreateReply::default()
                .content("Invalid time format. Please use HH:MM in 24h format, e.g. 08:30.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let user_id = ctx.author().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;

    if let Err(e) = add_reminder(user_id, Some(guild_id_i64), &kind_lc, time.trim(), note.as_deref(), private) {
        tracing::error!("Failed to save reminder: {}", e);
        ctx.send(
            poise::CreateReply::default()
                .content("Failed to save your reminder. Please try again later.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut msg = format!(
        "Saved a {} reminder at {} ({}).",
        kind_lc,
        time.trim(),
        if private { "private" } else { "public" }
    );
    if let Some(n) = &note { if !n.trim().is_empty() { msg.push_str(&format!(" Note: {}", n.trim())); } }

    ctx.send(
        poise::CreateReply::default()
            .content(msg)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
