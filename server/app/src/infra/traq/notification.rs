use crate::domain::notification::{NotificationService, StampNotification, WordNotification};
use anyhow::Result;
use traq::{
    apis::{configuration::Configuration, message_api::post_direct_message},
    models::PostMessageRequest,
};

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
            "「{}」\n{}",
            notification
                .matched_word_values
                .iter()
                .map(|w| w.0.as_str())
                .collect::<Vec<_>>()
                .join("」「"),
            format!("https://q.trap.jp/messages/{}", notification.message_uuid.0)
        );
        post_direct_message(
            &self.config,
            &notification.target_user_uuid.to_string(),
            Some(PostMessageRequest {
                content: message,
                embed: Some(false),
            }),
        )
        .await?;

        Ok(())
    }

    async fn send_stamp_notification(&self, _notification: StampNotification) -> Result<()> {
        unimplemented!()
    }
}
