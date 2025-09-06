use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use poise::serenity_prelude::FullEvent as Event;
use rand::prelude::IndexedRandom;
use crate::shared::types::{Data, Error};
use crate::shared::utils::special_user_id;

pub async fn on_event(
    ctx: &serenity::Context,
    event: &Event,
    _framework: FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Message { new_message } => {
            if new_message.author.bot { return Ok(()); }

            let bot_id = ctx.cache.current_user().id;

            let raw_message = new_message.content.clone();
            let mentions_bot = new_message.mentions.iter().any(|u| u.id == bot_id)
                || raw_message.contains(&format!("<@{}>", bot_id.get()))
                || raw_message.contains(&format!("<@!{}>", bot_id.get()));

            if !mentions_bot { return Ok(()); }

            let lower_case_message = raw_message.to_lowercase();

            let specific_response = if lower_case_message.contains("who are you") {
                Some("I'm Shaggy, your friendly shaggy ink cap Discord bot-shroom.".to_string())
            } else if lower_case_message.contains("good bot") {
                Some("Thanks! I do my best.".to_string())
            } else if lower_case_message.contains("bad bot") {
                Some("I'm still learning. How can I improve?".to_string())
            } else if lower_case_message.contains("thank") {
                Some("You're welcome!".to_string())
            } else if lower_case_message.contains("help") {
                Some("Need help? Try /help to see what I can do.".to_string())
            } else if lower_case_message.contains("meme of 2024") || lower_case_message.contains("massive") {
                Some("Massive. https://i.redd.it/31nha5vc6sge1.jpeg".to_string())
            } else {
                None
            };

            let reply = if let Some(resp) = specific_response {
                resp
            } else {
                let esme = special_user_id("ESME_USER_ID").unwrap_or_default();
                let shan = special_user_id("SHAN_USER_ID").unwrap_or_default();
                if new_message.author.id == esme {
                    format!("Salutations, {}, my liege.", new_message.author.display_name())
                } else if new_message.author.id == shan {
                    format!("Meowdy, {}", new_message.author.display_name())
                } else {
                    let greetings = ["Hi", "Hello", "Hey", "Heya", "Greetings", "Howdy"];
                    let mut rng = rand::rng();
                    let greet = greetings.choose(&mut rng).copied().unwrap_or("Hi");
                    format!("{}, {}!", greet, new_message.author.display_name())
                }
            };

            let _ = new_message.channel_id.say(ctx, reply).await;
        }
        _ => {}
    }

    Ok(())
}