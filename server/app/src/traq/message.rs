use anyhow::{Ok, Result};
use traq::apis::{configuration::Configuration, message_api::search_messages};

use super::aaa;

pub async fn collect_message() -> Result<()> {
    let configuration = Configuration {
        bearer_access_token: Some(aaa::ACCESS_TOKEN),
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
            Some(aaa::MESSAGE_LIMIT),
            Some(i),
            Some("createdAt"),
        )
        .await?;

        let _a = result.hits;
    }

    Ok(())
}
