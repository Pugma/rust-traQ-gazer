use crate::domain::{traq_message::TraqMessageUuid, traq_stamp::TraqStampName, word::WordValue};
use uuid::Uuid;

pub struct WordNotification {
    target_user_uuid: Uuid,
    matched_word_values: Vec<WordValue>,
    message_uuid: TraqMessageUuid,
}

pub struct StampNotification {
    target_user_uuid: Uuid,
    matched_stamp_names: Vec<TraqStampName>,
    message_uuid: TraqMessageUuid,
}

pub trait NotificationService {
    fn send_word_notification(&self, notification: WordNotification) -> Result<(), String>;
    fn send_stamp_notification(&self, notification: StampNotification) -> Result<(), String>;
}
