use chrono::NaiveTime;
use chrono_tz::Tz;
use poise::ChoiceParameter;
use crate::reminder::models::ReminderKind;
use crate::shared::db::{add_reminder, delete_reminder_owned, get_user_reminders_in_guild};
use crate::shared::types::{Context, Error};

/// Reminder-related commands
#[poise::command(slash_command, subcommands("register", "list", "delete"))]
pub async fn remind(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Register a reminder for medicine or food at a specific time (HH:MM, 24h)
#[poise::command(slash_command, guild_only)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Type of reminder (medicine, food, other)"] kind: ReminderKind,
    #[description = "Time in HH:MM (24h) e.g. 08:30"] time: String,
    #[description = "Optional note (e.g., medicine name)"] note: Option<String>,
    #[description = "Send this reminder privately via DM (instead of the public channel)?"] private: bool,
    #[description = "IANA timezone, e.g. Europe/Lisbon, America/New_York (default: UTC)"] timezone: Option<String>,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        None => {
            ctx.say("This command can only be used in a server.").await?;
            return Ok(());
        }
    };

    let kind_lc = kind.name().to_lowercase();

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

    let tz_str = timezone.as_deref().unwrap_or("UTC").trim();
    // Validate timezone
    if tz_str.parse::<Tz>().is_err() {
        ctx.send(
            poise::CreateReply::default()
                .content("Invalid timezone. Please use a valid IANA timezone like Europe/Lisbon or America/New_York.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let user_id = ctx.author().id.get() as i64;
    let guild_id_i64 = guild_id.get() as i64;

    if let Err(e) = add_reminder(user_id, Some(guild_id_i64), &kind_lc, time.trim(), note.as_deref(), private, tz_str) {
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
        "Saved a {} reminder at {} ({}), timezone: {}.",
        kind_lc,
        time.trim(),
        if private { "private" } else { "public" },
        tz_str
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

/// List your reminders in this server
#[poise::command(slash_command, guild_only)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        None => {
            ctx.say("This command can only be used in a server.").await?;
            return Ok(());
        }
    };
    let user_id = ctx.author().id.get() as i64;
    let reminders = match get_user_reminders_in_guild(user_id, guild_id.get() as i64) {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch reminders: {}", e);
            ctx.send(
                poise::CreateReply::default()
                    .content("Failed to fetch your reminders. Please try again later.")
                    .ephemeral(true),
            ).await?;
            return Ok(());
        }
    };

    if reminders.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("You have no reminders in this server.")
                .ephemeral(true),
        ).await?;
        return Ok(());
    }

    let mut out = String::new();
    out.push_str("Your reminders:\n");
    for r in reminders {
        let note = r.note.as_deref().unwrap_or("");
        let privacy = if r.private { "private" } else { "public" };
        if note.is_empty() {
            out.push_str(&format!("- ID {}: {} at {} ({}, tz: {})\n", r.id, r.kind, r.time, privacy, r.timezone));
        } else {
            out.push_str(&format!("- ID {}: {} at {} ({}, tz: {}), note: {}\n", r.id, r.kind, r.time, privacy, r.timezone, note));
        }
    }

    ctx.send(poise::CreateReply::default().content(out).ephemeral(true)).await?;
    Ok(())
}

/// Delete one of your reminders by ID
#[poise::command(slash_command, guild_only)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Reminder ID to delete (see /remind list)"] id: i64,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(g) => g,
        None => {
            ctx.say("This command can only be used in a server.").await?;
            return Ok(());
        }
    };
    let user_id = ctx.author().id.get() as i64;

    match delete_reminder_owned(id, user_id, guild_id.get() as i64) {
        Ok(true) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Deleted reminder {}.", id))
                    .ephemeral(true),
            ).await?;
        }
        Ok(false) => {
            ctx.send(
                poise::CreateReply::default()
                    .content("No reminder found with that ID that belongs to you in this server.")
                    .ephemeral(true),
            ).await?;
        }
        Err(e) => {
            tracing::error!("Failed to delete reminder: {}", e);
            ctx.send(
                poise::CreateReply::default()
                    .content("Failed to delete reminder. Please try again later.")
                    .ephemeral(true),
            ).await?;
        }
    }

    Ok(())
}
