use anyhow::{Ok, Result};
use log::{debug, error, info};
use traq::apis::{configuration::Configuration, message_api::search_messages};

use super::MESSAGE_LIMIT;

pub(super) async fn collect(config: &Configuration) -> Result<()> {
    if let Some(token) = config.bearer_access_token.clone() {
        debug!("bot_access_token is Some object");
        if token == *"" {
            error!("bot access token was empty");
            return Ok(());
        };
    }

    for i in 0..1 {
        let result = search_messages(
            config,
            None,
            Some("2024-09-30T22:30:00.000000".to_string()),
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
            None,
            Some(MESSAGE_LIMIT),
            Some(MESSAGE_LIMIT * i),
            Some("-createdAt"),
        )
        .await;

        if result.is_err() {
            error!("Couldn't get message");
            return Ok(());
        }

        let result = result.unwrap();
        let hit_messages = result.hits;
        info!("{}", hit_messages.len());
        info!("{}", hit_messages[0].id);
    }

    Ok(())
}
