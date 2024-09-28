pub mod message;
use std::{env, sync::LazyLock};

use anyhow::{Ok, Result};
use message::collect_message;
use tokio::{time, time::Duration};
use traq::apis::configuration::Configuration;

pub const MESSAGE_LIMIT: i32 = 100;

static CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    bearer_access_token: Some(env::var("BOT_ACCESS_TOKEN").expect("msg")),
    ..Default::default()
});

pub async fn start_polling() -> Result<()> {
    tokio::spawn(async {
        loop {
            // 3 分おきに実行
            let mut interval = time::interval(Duration::new(180, 0));
            interval.tick().await;

            tokio::spawn(async {
                println!("start polling ...");
                let _ = collect_message(&CONFIG).await;
            });
        }
    });

    Ok(())
}
