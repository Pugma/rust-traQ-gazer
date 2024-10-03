use anyhow::{Ok, Result};
use log::info;
use std::{env, sync::LazyLock};
use tokio::{time, time::Duration};
use traq::apis::configuration::Configuration;

use crate::repo::Repository;

pub mod message;
pub const MESSAGE_LIMIT: i32 = 100;

static CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    bearer_access_token: Some(
        env::var("BOT_ACCESS_TOKEN").expect("Couldn't find BOT_ACCESS_TOKEN"),
    ),
    ..Default::default()
});

pub async fn start_polling(repo: Repository) -> Result<()> {
    tokio::spawn(async move {
        // 3 分おきに実行
        let mut interval = time::interval(Duration::new(180, 0));
        interval.tick().await;

        loop {
            interval.tick().await;

            let repo = repo.clone();
            tokio::spawn(async move {
                info!("start polling ...");
                let _ = message::collect(&repo, &CONFIG).await;
            });
        }
    })
    .await?;

    Ok(())
}
