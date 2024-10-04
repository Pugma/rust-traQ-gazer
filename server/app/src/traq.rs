use anyhow::Result;
use chrono::{SecondsFormat, Utc};
use log::{error, info};
use std::{env, sync::LazyLock};
use tokio::{time, time::Duration};
use traq::apis::configuration::Configuration;

use crate::repo::Repository;

pub mod message;
pub const MESSAGE_LIMIT: i32 = 100;
const POLLING_INTERVAL_SEC: u64 = 180;

static CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    bearer_access_token: Some(
        env::var("BOT_ACCESS_TOKEN").expect("Couldn't find BOT_ACCESS_TOKEN"),
    ),
    ..Default::default()
});

pub async fn start_polling(repo: Repository) -> Result<()> {
    tokio::spawn(async move {
        // run polling at even intervals
        let mut interval = time::interval(Duration::new(POLLING_INTERVAL_SEC, 0));
        interval.tick().await;

        // get last timestamp when this app restarts
        let mut last_checkpoint = if let Ok(point) = repo.get_time().await {
            point
        } else {
            error!("Couldn't get last checkpoint!");
            Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true)
        };

        loop {
            interval.tick().await;

            info!("start polling ...");
            let _ = message::collect(&repo, &CONFIG, &mut last_checkpoint).await;
        }
    })
    .await?;

    Ok(())
}
