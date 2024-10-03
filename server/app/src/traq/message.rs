use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use log::{debug, error, info};
use traq::apis::{configuration::Configuration, message_api::search_messages};

use crate::repo::Repository;

use super::MESSAGE_LIMIT;

pub(super) async fn collect(repo: &Repository, config: &Configuration) -> Result<()> {
    if let Some(token) = config.bearer_access_token.clone() {
        debug!("bot_access_token is Some object");
        if token == *"" {
            error!("bot access token was empty");
            return Ok(());
        };
    }

    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true);

    for i in 0..1 {
        let result = search_messages(
            config,
            None,
            Some("2024-09-30T22:30:00.000000Z".to_string()),
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
            Some(MESSAGE_LIMIT * i),
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
        info!("{}", hit_messages.len());
        info!("{}", hit_messages[0].id);
    }

    repo.record_time(now).await?;

    Ok(())
}
