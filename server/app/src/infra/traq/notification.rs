use crate::domain::notification::{NotificationService, StampNotification, WordNotification};
use anyhow::Result;
use traq::apis::configuration::Configuration;

pub struct TraqNotificationService {
    config: Configuration,
}

impl TraqNotificationService {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }
}

impl NotificationService for TraqNotificationService {
    async fn send_word_notification(&self, notification: WordNotification) -> Result<()> {
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
            .await?;
        Ok(())
    }

    async fn send_stamp_notification(&self, _notification: StampNotification) -> Result<()> {
        unimplemented!()
    }
}
