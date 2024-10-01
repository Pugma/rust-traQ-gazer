use anyhow::{Ok, Result};
use log::info;
use traq::apis::{configuration::Configuration, message_api::search_messages};

pub(super) async fn collect_message(config: &Configuration) -> Result<()> {
    if let Some(token) = config.bearer_access_token.clone() {
        if token == "".to_string() {
            info!("bot access token was empty");
            return Ok(());
        };
    }

    for i in 0..1 {
        let result = search_messages(
            config,
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
            None,
            None,
            Some(super::MESSAGE_LIMIT),
            Some(i),
            Some("createdAt"),
        )
        .await.expect("failed to traQ への負荷");

        let hit_messages = result.hits;
        info!("{}", hit_messages.len())
    }

    Ok(())
}
