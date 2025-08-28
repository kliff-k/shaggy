use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Attachment, CreateEmbed, GuildChannel};

use crate::shared::types::{Context, Error};

/// Shows help information
#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let configuration = poise::builtins::HelpConfiguration {
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), configuration).await?;
    Ok(())
}

/// Allows an administrator to send embedded/formated messages to specific channels.
#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "Channel to send the embed to"] channel: GuildChannel,
    #[description = "Title of the embed"] title: String,
    #[description = "The main content of the embed"] description: String,
    #[description = "Image to attach to the embed"] image: Option<Attachment>,
    #[description = "Thumbnail to attach to the embed"] thumbnail: Option<Attachment>,
    #[description = "Hex color for the embed (e.g. #0000FF)"] color: Option<String>,
) -> Result<(), Error> {
    let colour = if let Some(hex) = color {
        let hex_code = hex.strip_prefix('#').unwrap_or(&hex);
        match u32::from_str_radix(hex_code, 16) {
            Ok(c) => serenity::Colour::new(c),
            Err(_) => {
                ctx.send(
                    poise::CreateReply::default()
                        .content(
                            "Invalid hex color format. Please use a valid hex code (e.g., #FF0000).",
                        )
                        .ephemeral(true),
                )
                .await?;
                return Ok(());
            }
        }
    } else {
        serenity::Colour::default()
    };

    let description_with_breaks = description.replace("\\n", "\n");

    let mut embed_builder = CreateEmbed::new()
        .title(title)
        .description(description_with_breaks)
        .colour(colour);

    if let Some(image_attachment) = image {
        embed_builder = embed_builder.image(image_attachment.url);
    }

    if let Some(thumbnail_attachment) = thumbnail {
        embed_builder = embed_builder.thumbnail(thumbnail_attachment.url);
    }

    let message_builder = serenity::builder::CreateMessage::new().embed(embed_builder);

    if let Err(why) = channel.send_message(ctx.http(), message_builder).await {
        println!("Error sending embed: {:?}", why);
        ctx.send(
            poise::CreateReply::default()
                .content("Failed to send the embed. Please check my permissions in that channel.")
                .ephemeral(true),
        )
        .await?;
    } else {
        ctx.send(
            poise::CreateReply::default()
                .content("Embed sent successfully!")
                .ephemeral(true),
        )
        .await?;
    }

    Ok(())
}
