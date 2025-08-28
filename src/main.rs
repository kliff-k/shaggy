use std::env;

use poise::serenity_prelude as serenity;
use tracing::{error, info};

use shaggy::shared::{embed, help};
use shaggy::music::music;
use shaggy::recipe::recipe;
use shaggy::shared::scheduler::setup_daily_recipe_scheduler;
use shaggy::shared::types::{Data, Error};
use shaggy::shared::db::init_db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let recipe_channel_id = env::var("DAILY_RECIPE_CHANNEL_ID")
        .expect("Expected DAILY_RECIPE_CHANNEL_ID in the environment")
        .parse::<u64>()
        .expect("DAILY_RECIPE_CHANNEL_ID must be a valid number");
    let recipe_channel = serenity::ChannelId::new(recipe_channel_id);

    let schedule_str = env::var("DAILY_RECIPE_SCHEDULE")
        .expect("Expected DAILY_RECIPE_SCHEDULE in the environment");

    init_db()?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help(), embed(), recipe(), music()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let schedule = schedule_str.clone();
            Box::pin(async move {
                info!("Registering commands globally...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Logged in as {}", _ready.user.name);

                setup_daily_recipe_scheduler(ctx, &schedule, recipe_channel).await?;

                Ok(Data {})
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    info!("Starting the bot...");
    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}