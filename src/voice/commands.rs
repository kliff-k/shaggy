use crate::shared::types::{Context, Error};
use crate::shared::db::{tts_is_signed, tts_signup, tts_signout};
use crate::voice::utils::{find_user_voice_channel};

/// Text-to-Speech features
#[poise::command(slash_command, subcommands("signup", "signout", "join", "leave"))]
pub async fn tts(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Opt-in to TTS reading your messages while you are in a voice call
#[poise::command(slash_command)]
pub async fn signup(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() { Some(g) => g, None => {
        ctx.say("This command can only be used in a server.").await?; return Ok(());
    }};
    let user_id = ctx.author().id;

    if tts_is_signed(user_id.get() as i64, guild_id.get() as i64)? {
        ctx.say("You are already signed up for TTS.").await?;
        return Ok(());
    }

    tts_signup(user_id.get() as i64, guild_id.get() as i64)?;
    ctx.say("You are now signed up for TTS. I will speak your messages when you chat in your current voice channel.").await?;
    Ok(())
}

/// Opt-out from TTS
#[poise::command(slash_command)]
pub async fn signout(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() { Some(g) => g, None => {
        ctx.say("This command can only be used in a server.").await?; return Ok(());
    }};
    let user_id = ctx.author().id;

    if !tts_is_signed(user_id.get() as i64, guild_id.get() as i64)? {
        ctx.say("You are not signed up for TTS.").await?;
        return Ok(());
    }

    tts_signout(user_id.get() as i64, guild_id.get() as i64)?;
    ctx.say("You have been removed from TTS.").await?;
    Ok(())
}

/// Ask the bot to join your current voice channel (required for TTS)
#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() { Some(g) => g, None => {
        ctx.say("This command can only be used in a server.").await?; return Ok(());
    }};
    let user_id = ctx.author().id;

    let Some(channel_id) = find_user_voice_channel(ctx.serenity_context(), guild_id, user_id).await else {
        ctx.say("You must be connected to a voice channel.").await?;
        return Ok(());
    };

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird Voice client placed in at initialisation.");

    match manager.join(guild_id, channel_id).await {
        Ok(_call_lock) => {
            ctx.say(format!("Joined <#{}>.", channel_id.get())).await?;
        }
        Err(e) => {
            ctx.say("Failed to join the voice channel.").await?;
            tracing::error!("Songbird join error: {}", e);
        }
    }

    Ok(())
}

/// Ask the bot to leave the current voice channel
#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() { Some(g) => g, None => {
        ctx.say("This command can only be used in a server.").await?; return Ok(());
    }};

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird Voice client placed in at initialisation.");
    if manager.get(guild_id).is_none() {
        ctx.say("I'm not in a voice channel.").await?;
        return Ok(());
    }

    if let Err(e) = manager.leave(guild_id).await {
        tracing::error!("Songbird leave error: {}", e);
        ctx.say("Failed to leave the voice channel.").await?;
    } else {
        ctx.say("Left the voice channel.").await?;
    }

    Ok(())
}
