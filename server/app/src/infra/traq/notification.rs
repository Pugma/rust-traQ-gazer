use crate::domain::notification::{NotificationService, StampNotification, WordNotification};
use traq::apis::{channel_api, configuration::Configuration};
use uuid::Uuid;

pub struct TraqNotificationService {
    config: Configuration,
}

impl TraqNotificationService {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }
}

use async_trait::async_trait;

#[async_trait]
impl NotificationService for TraqNotificationService {
    async fn send_word_notification(&self, notification: WordNotification) -> Result<(), String> {
        let message = format!(
            "以下の単語がマッチしました: {}\n{}",
            notification
                .matched_word_values
                .iter()
                .map(|w| w.0.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            format!(
                "https://q.trap.jp/messages/{}",
                notification.message_uuid.0
            )
        );
        let user_id = notification.target_user_uuid;
        let embed = true;
        let body = traq::models::PostMessageRequest {
            content: message,
            embed: Some(embed),
        };
        traq::apis::user_api::post_direct_message(&self.config, &user_id.to_string(), Some(body))
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn send_stamp_notification(
        &self,
        _notification: StampNotification,
    ) -> Result<(), String> {
        unimplemented!()
    }
}
