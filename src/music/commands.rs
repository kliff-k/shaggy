use std::path::Path;
use tokio::fs;
use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CreateAttachment, CreateEmbed};
use poise::ChoiceParameter;
use crate::music::models::{FinalFantasyExpansion, Game, KingdomHeartsTitle};
use crate::music::utils::get_random_song;
use crate::shared::types::{Context, Error};

/// Play music from the collection.
#[poise::command(slash_command, subcommands("music_random", "music_ff", "music_kh"))]
pub async fn music(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/music random`, `/music ff`, or `/music kh`.").await?;
    Ok(())
}

/// Get a random song from the main music folder.
#[poise::command(slash_command, rename = "random")]
pub async fn music_random(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let music_folder = "/mnt/nas/Media/Music/Game/";
    match get_random_song(music_folder).await {
        Ok(Some(song_path)) => {
            let file_name = song_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown_song.mp3".to_string());

            let file_content = fs::read(&song_path)
                .await
                .with_context(|| format!("Failed to read song file: {}", song_path.display()))?;

            let attachment = CreateAttachment::bytes(file_content, &file_name);

            let embed = CreateEmbed::new()
                .title("🎶 Random Song")
                .description(format!("Playing: **{}**", file_name))
                .color(serenity::Colour::PURPLE);

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .attachment(attachment),
            )
                .await?;
        }
        Ok(None) => {
            ctx.say(format!(
                "Couldn't find any MP3 songs in the `{}` folder.",
                music_folder
            ))
                .await?;
        }
        Err(e) => {
            tracing::error!("Error getting random song: {:?}", e);
            ctx.say("An error occurred while trying to get a random song.")
                .await?;
        }
    }
    Ok(())
}

/// Get a random song from the Final Fantasy folder.
#[poise::command(slash_command, rename = "ff")]
pub async fn music_ff(
    ctx: Context<'_>,
    #[description = "Which expansion to pick music from"] expansion: FinalFantasyExpansion,
) -> Result<(), Error> {
    let base_folder = Game::FF.folder_name();
    let expansion_folder = expansion.folder_name();
    let path_str = format!(
        "/mnt/nas/Media/Music/Game/{}/{}",
        base_folder, expansion_folder
    );
    let music_dir = Path::new(&path_str);

    match get_random_song(music_dir).await {
        Ok(Some(song_path)) => {
            let file_name = song_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown_song.mp3".to_string());

            let file_content = fs::read(&song_path)
                .await
                .with_context(|| format!("Failed to read song file: {}", song_path.display()))?;

            let attachment = CreateAttachment::bytes(file_content, &file_name);

            let embed = CreateEmbed::new()
                .title("🎶 Random Song")
                .description(format!("Playing: **{}**", file_name))
                .color(serenity::Colour::PURPLE);

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .attachment(attachment),
            )
                .await?;
        }
        Ok(None) => {
            ctx.say(format!(
                "Couldn't find any songs in the {} folder for Final Fantasy ({}).",
                expansion_folder,
                expansion.name()
            ))
                .await?;
        }
        Err(e) => {
            tracing::error!("Error getting random song: {}", e);
            ctx.say("An error occurred while trying to find a song.")
                .await?;
        }
    }
    Ok(())
}

/// Get a random song from the Kingdom Hearts folder.
#[poise::command(slash_command, rename = "kh")]
pub async fn music_kh(
    ctx: Context<'_>,
    #[description = "Which title to pick music from"] title: KingdomHeartsTitle,
) -> Result<(), Error> {
    let base_folder = Game::KH.folder_name();
    let title_folder = title.folder_name();
    let path_str = format!(
        "/mnt/nas/Media/Music/Game/{}/{}",
        base_folder, title_folder
    );
    let music_dir = Path::new(&path_str);

    match get_random_song(music_dir).await {
        Ok(Some(song_path)) => {
            let file_name = song_path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown_song.mp3".to_string());

            let file_content = fs::read(&song_path)
                .await
                .with_context(|| format!("Failed to read song file: {}", song_path.display()))?;

            let attachment = CreateAttachment::bytes(file_content, &file_name);

            let embed = CreateEmbed::new()
                .title("🎶 Random Song")
                .description(format!("Playing: **{}**", file_name))
                .color(serenity::Colour::PURPLE);

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .attachment(attachment),
            )
                .await?;
        }
        Ok(None) => {
            ctx.say(format!(
                "Couldn't find any songs in the {} folder for Kingdom Hearts ({}).",
                title_folder,
                title.name()
            ))
                .await?;
        }
        Err(e) => {
            tracing::error!("Error getting random song: {}", e);
            ctx.say("An error occurred while trying to find a song.")
                .await?;
        }
    }
    Ok(())
}