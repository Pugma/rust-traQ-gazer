use crate::{infra::repo::Repository, usecase::message_poller::MessagePoller};
use chrono::{DateTime, SecondsFormat, Utc};
use log::{error, info};
use std::time::Duration;
use tokio::time;
use traq::apis::{configuration::Configuration, message_api::search_messages};

const POLLING_INTERVAL_SEC: u64 = 180;
const MESSAGE_LIMIT: i32 = 100;

pub struct TraqMessagePoller {
    repo: Repository,
}

impl TraqMessagePoller {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    async fn poll_messages(
        &self,
        last_checkpoint: &mut DateTime<Utc>,
    ) -> Result<Vec<String>, String> {
        let cfg = Configuration {
            bearer_access_token: Some(std::env::var("BOT_ACCESS_TOKEN").unwrap_or_default()),
            ..Default::default()
        };

        let now = Utc::now();

        for page in 0.. {
            let result = search_messages(
                &cfg,
                None,
                Some(last_checkpoint.to_rfc3339_opts(SecondsFormat::Nanos, true)),
                Some(now.to_rfc3339_opts(SecondsFormat::Nanos, true)),
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
                Some(MESSAGE_LIMIT * page),
                Some("-createdAt"),
            )
            .await;

            let result = if let Ok(result) = result {
                result
            } else {
                error!("Couldn't get messages from traQ!");
                break;
            };

            let hit_messages = result.hits;
            info!("Collected {} messages", hit_messages.len());

            if MESSAGE_LIMIT * (page + 1) >= result.total_hits as i32 {
                if hit_messages.is_empty() {
                    *last_checkpoint = now;
                    info!("Updated last_checkpoint = {}", *last_checkpoint);
                    break;
                }

                // 最新メッセージのタイムスタンプを取得
                *last_checkpoint = hit_messages
                    .last()
                    .unwrap()
                    .created_at
                    .clone()
                    .parse()
                    .unwrap_or_else(|_| {
                        error!("Failed to parse last message timestamp");
                        now
                    });
                info!("Updated last_checkpoint = {}", *last_checkpoint);
                break;
            }
        }

        // Simulate message collection
        Ok(vec!["message1".into(), "message2".into()])
    }
}

impl MessagePoller for TraqMessagePoller {
    async fn poll_messages(&self) -> Result<(), String> {
        let mut interval = time::interval(Duration::from_secs(POLLING_INTERVAL_SEC));

        let mut last_checkpoint = match self.repo.get_time().await {
            Ok(point) => point,
            Err(_) => {
                error!("Couldn't get last checkpoint!");
                Utc::now()
            }
        };

        loop {
            interval.tick().await;

            info!("start polling ...");
            last_checkpoint = Utc::now();

            self.poll_messages(&mut last_checkpoint).await?;

            self.repo.record_time(last_checkpoint).await;
        }
    }
}
