use crate::{
    config::traq::TRAQ_CONFIG, domain::traq_message::TraqMessage,
    usecase::message_poller::MessagePoller,
};
use chrono::{DateTime, SecondsFormat, Utc};
use log::{error, info};
use traq::apis::{configuration::Configuration, message_api::search_messages};

const MESSAGE_LIMIT: i32 = 100;

pub struct TraqMessageCollector {}

impl TraqMessageCollector {
    pub fn new() -> Self {
        Self {}
    }
}

impl MessagePoller for TraqMessageCollector {
    async fn collect_messages(
        &self,
        last_checkpoint: &mut DateTime<Utc>,
    ) -> Result<Vec<TraqMessage>, String> {
        let cfg = Configuration {
            bearer_access_token: Some(TRAQ_CONFIG.bot_access_token.clone()),
            ..Default::default()
        };

        let now = Utc::now();

        let messages = vec![];

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
                    .clone()
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

        Ok(messages)
    }
}
