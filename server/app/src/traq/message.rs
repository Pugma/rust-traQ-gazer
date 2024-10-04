use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use log::{debug, error, info};
use traq::apis::{configuration::Configuration, message_api::search_messages};

use crate::repo::Repository;

use super::MESSAGE_LIMIT;

pub(super) async fn collect(
    repo: &Repository,
    config: &Configuration,
    checkpoint: &mut String,
) -> Result<()> {
    if let Some(token) = config.bearer_access_token.clone() {
        debug!("bot_access_token is Some object");
        if token == *"" {
            error!("bot access token was empty");
            return Ok(());
        };
    }

    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true);

    for page in 0.. {
        let result = search_messages(
            config,
            None,
            Some(checkpoint.clone()),
            Some(now.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(MESSAGE_LIMIT),
            Some(MESSAGE_LIMIT * page),
            Some("-createdAt"),
        )
        .await;

        let result = if let Ok(result) = result {
            result
        } else {
            error!("Couldn't get messages from traQ!");
            break;
        };

        let hit_messages = result.hits;
        info!("Collected {} messages", hit_messages.len());

        // check whether all messages are retrieved
        if MESSAGE_LIMIT * (page + 1) >= result.total_hits as i32 {
            if hit_messages.is_empty() {
                *checkpoint = now;
                info!("Updated last_checkpoint = {}", *checkpoint);
                break;
            }

            // get the timestamp from the latest message
            *checkpoint = hit_messages.last().unwrap().created_at.clone();
            info!("Updated last_checkpoint = {}", *checkpoint);
            break;
        }
    }

    repo.record_time(checkpoint.clone()).await?;

    Ok(())
}
