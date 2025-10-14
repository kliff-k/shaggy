use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use poise::serenity_prelude as serenity;
use tokio::fs;
use tokio::task;
use crate::shared::types::Error;

pub async fn find_user_voice_channel(ctx: &serenity::Context, guild_id: serenity::GuildId, user_id: serenity::UserId) -> Option<serenity::ChannelId> {
    let guild = guild_id.to_guild_cached(ctx)?;
    let voice_states = &guild.voice_states;
    let state = voice_states.get(&user_id)?;
    state.channel_id
}

pub async fn synthesize_to_wav(text: &str) -> Result<PathBuf, Error> {
    // Create a temp file path
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let tmp_path = std::env::temp_dir().join(format!("shaggy_tts_{}.wav", now));

    let text = text.to_string();
    let out = tmp_path.clone();

    // Try pico2wave first, fallback to espeak
    task::spawn_blocking(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // prefer pico2wave for naturalness
        let pico = std::process::Command::new("pico2wave")
            .args(["-w", out.to_string_lossy().as_ref(), &text])
            .output();
        match pico {
            Ok(output) if output.status.success() => Ok(()),
            _ => {
                // try espeak-ng
                let es = std::process::Command::new("espeak-ng")
                    .args(["-w", out.to_string_lossy().as_ref(), &text])
                    .output();
                match es {
                    Ok(output) if output.status.success() => Ok(()),
                    Ok(f) => Err(format!("espeak-ng failed: status {:?}", f.status).into()),
                    Err(e) => Err(format!("No TTS engine available: {}", e).into()),
                }
            }
        }
    }).await??;

    Ok(tmp_path)
}

pub async fn cleanup_file(path: PathBuf) {
    let _ = fs::remove_file(path).await;
}
