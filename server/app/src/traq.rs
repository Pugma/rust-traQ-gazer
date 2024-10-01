use anyhow::{Ok, Result};
use log::info;
use message::collect_message;
use std::{env, sync::LazyLock};
use tokio::{time, time::Duration};
use traq::apis::configuration::Configuration;

pub mod message;
pub const MESSAGE_LIMIT: i32 = 100;

static CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    bearer_access_token: Some(
        env::var("BOT_ACCESS_TOKEN").expect("Couldn't find BOT_ACCESS_TOKEN"),
    ),
    ..Default::default()
});

pub async fn start_polling() -> Result<()> {
    tokio::spawn(async {
        // 3 分おきに実行
        let mut interval = time::interval(Duration::new(180, 0));
        interval.tick().await;

        loop {
            interval.tick().await;

            tokio::spawn(async {
                info!("start polling ...");
                let _ = collect_message(&CONFIG).await;
            });
        }
    })
    .await?;

    Ok(())
}
