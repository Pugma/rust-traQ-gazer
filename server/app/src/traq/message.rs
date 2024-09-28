use anyhow::{Ok, Result};
use traq::apis::{configuration::Configuration, message_api::search_messages};

pub async fn collect_message() -> Result<()> {
    let configuration = Configuration {
        bearer_access_token: Some(super::ACCESS_TOKEN.clone()),
        ..Default::default()
    };

    for i in 0.. {
        let result = search_messages(
            &configuration,
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

        let _a = result.hits;
    }

    Ok(())
}
