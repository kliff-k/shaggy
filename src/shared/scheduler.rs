use poise::serenity_prelude as serenity;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

use crate::shared::types::Error;
use crate::recipe::utils::{get_random_meal, format_meal};
use crate::shared::db::{was_recipe_sent, log_recipe_sent};

pub async fn setup_daily_recipe_scheduler(
    ctx: &serenity::Context,
    schedule: &str,
    channel: serenity::ChannelId,
) -> Result<(), Error> {
    info!("Setting up daily recipe scheduler...");

    let http_client = ctx.http.clone();
    let reqwest_client = reqwest::Client::new();
    let scheduler = JobScheduler::new().await?;

    let job = Job::new_async(schedule, move |_uuid, _lock| {
        let http = http_client.clone();
        let reqwest = reqwest_client.clone();
        let channel = channel;

        Box::pin(async move {
            info!("Running scheduled job: Sending daily recipe...");

            let mut tries = 0u8;
            let mut chosen: Option<(crate::recipe::models::Meal, String)> = None;
            let mut is_repeat = false;

            while tries < 5 {
                match get_random_meal(&reqwest).await {
                    Ok(Some(meal)) => {
                        let recipe_id = meal
                            .id
                            .clone()
                            .or_else(|| meal.extra.get("idMeal").and_then(|v| v.clone()))
                            .unwrap_or_else(|| meal.name.clone());

                        match was_recipe_sent(&recipe_id) {
                            Ok(true) => {
                                tries += 1;
                                if tries >= 5 {
                                    is_repeat = true;
                                    chosen = Some((meal, recipe_id));
                                    break;
                                }
                                continue;
                            }
                            Ok(false) => {
                                chosen = Some((meal, recipe_id));
                                break;
                            }
                            Err(e) => {
                                error!("DB check failed: {}", e);
                                // best-effort: proceed as if not sent
                                chosen = Some((meal, recipe_id));
                                break;
                            }
                        }
                    }
                    Ok(None) => {
                        warn!("Scheduled job: No recipe received from API.");
                        break;
                    }
                    Err(e) => {
                        error!("Scheduled job: Failed to get random recipe: {}", e);
                        break;
                    }
                }
            }

            if let Some((meal, recipe_id)) = chosen {
                let embed = format_meal(&meal, true, is_repeat);
                let builder = serenity::CreateMessage::new().embed(embed);
                if let Err(e) = channel.send_message(&http, builder).await {
                    error!("Failed to send daily recipe: {}", e);
                } else {
                    if let Err(e) = log_recipe_sent(&recipe_id, &meal.name) {
                        warn!("Failed to log sent recipe: {}", e);
                    }
                    info!("Successfully sent daily recipe to channel {}", channel);
                }
            }
        })
    })?;

    scheduler.add(job).await?;
    info!("Daily recipe job added with schedule: {}", schedule);

    tokio::spawn(async move {
        if let Err(e) = scheduler.start().await {
            error!("Scheduler failed to start: {}", e);
        }
    });
    info!("Scheduler started.");

    Ok(())
}
