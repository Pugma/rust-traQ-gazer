use chrono::{DateTime, SecondsFormat, Utc};
use log::{debug, error, info};
use traq::apis::message_api::search_messages;

use std::sync::Arc;

use crate::{
    config::traq::TRAQ_CONFIG,
    domain::{
        traq_message::{TraqMessage, TraqMessageStamp, TraqMessageUuid},
        user::UserRepository,
    },
    usecase::message_poller::MessagePoller,
};

const MESSAGE_LIMIT: i32 = 100;

pub struct TraqMessageCollector<U>
where
    U: UserRepository,
{
    user_repo: Arc<U>,
}

impl<U> TraqMessageCollector<U>
where
    U: UserRepository,
{
    pub fn new(user_repo: Arc<U>) -> Self {
        Self { user_repo }
    }
}

impl<U> MessagePoller for TraqMessageCollector<U>
where
    U: UserRepository + Send + Sync,
{
    async fn collect_messages(
        &self,
        last_checkpoint: &mut DateTime<Utc>,
    ) -> Result<Vec<TraqMessage>, String> {
        let cfg = TRAQ_CONFIG.options();

        let now = Utc::now();

        let mut messages = vec![];

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
                debug!(
                    "search_messages succeeded: total_hits={}, hits={}",
                    result.total_hits,
                    result.hits.len()
                );
                result
            } else {
                error!("Couldn't get messages from traQ!");
                break;
            };

            let hit_messages = result.hits;
            info!("Collected {} messages", hit_messages.len());

            for message in hit_messages.iter() {
                let user = match self.user_repo.find_by_traq_uuid(message.user_id).await {
                    Ok(user) => user,
                    Err(e) => {
                        log::error!("Failed to find user by traq_uuid: {}", e);
                        continue;
                    }
                };
                let mut stamps = Vec::new();
                for s in message.stamps.iter() {
                    let user = match self.user_repo.find_by_traq_uuid(s.user_id).await {
                        Ok(user) => user,
                        Err(e) => {
                            log::error!("Failed to find user by traq_uuid: {}", e);
                            continue;
                        }
                    };
                    stamps.push(TraqMessageStamp::new(s.stamp_id, user.id));
                }
                let traq_message = TraqMessage::new(
                    TraqMessageUuid::new(message.id),
                    user.id,
                    message.content.clone(),
                    stamps,
                );
                messages.push(traq_message);
            }

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
