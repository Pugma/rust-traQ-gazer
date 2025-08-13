use std::{env, sync::LazyLock};
use traq::apis::configuration::Configuration;

pub mod message;
pub mod message_poller_impl;
pub mod notification;

static CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    bearer_access_token: Some(
        env::var("BOT_ACCESS_TOKEN").expect("Couldn't find BOT_ACCESS_TOKEN"),
    ),
    ..Default::default()
});
