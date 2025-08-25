use crate::domain::{traq_message::TraqMessageUuid, traq_stamp::TraqStampName, word::WordValue};
use anyhow::Result;
use uuid::Uuid;

pub struct WordNotification {
    pub target_user_uuid: Uuid,
    pub matched_word_values: Vec<WordValue>,
    pub message_uuid: TraqMessageUuid,
}

impl WordNotification {
    pub fn new(
        target_user_uuid: Uuid,
        matched_word_values: Vec<WordValue>,
        message_uuid: TraqMessageUuid,
    ) -> Self {
        Self {
            target_user_uuid,
            matched_word_values,
            message_uuid,
        }
    }
}

pub struct StampNotification {
    target_user_uuid: Uuid,
    matched_stamp_names: Vec<TraqStampName>,
    message_uuid: TraqMessageUuid,
}
impl StampNotification {
    pub fn new(
        target_user_uuid: Uuid,
        matched_stamp_names: Vec<TraqStampName>,
        message_uuid: TraqMessageUuid,
    ) -> Self {
        Self {
            target_user_uuid,
            matched_stamp_names,
            message_uuid,
        }
    }

    pub fn target_user_uuid(&self) -> Uuid {
        self.target_user_uuid
    }
    pub fn matched_stamp_names(&self) -> &[TraqStampName] {
        &self.matched_stamp_names
    }
    pub fn message_uuid(&self) -> &TraqMessageUuid {
        &self.message_uuid
    }
}

pub trait NotificationService {
    async fn send_word_notification(&self, notification: WordNotification) -> Result<()>;
    async fn send_stamp_notification(&self, notification: StampNotification) -> Result<()>;
}
