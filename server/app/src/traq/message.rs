use anyhow::{Ok, Result};
use traq::apis::{configuration::Configuration, message_api::search_messages};

pub(super) async fn collect_message(config: &Configuration) -> Result<()> {
    for i in 0.. {
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
        .await?;

        let hit_messages = result.hits;
        println!("{}", hit_messages.len())
    }

    Ok(())
}
