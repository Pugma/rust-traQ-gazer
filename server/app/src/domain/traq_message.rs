use crate::domain::{traq_stamp::TraqStampName, user::UserId, word::WordValue};
use uuid::Uuid;

pub struct TraqMessageUuid(Uuid);

pub struct TraqMessage {
    uuid: TraqMessageUuid,
    author_user_id: UserId,
    content: String,
    stamps: Vec<TraqMessageStamp>,
}

pub struct TraqMessageStamp {
    stamp_uuid: Uuid,
    user_id: UserId,
}

pub trait TraqMessageService {
    async fn find_matching_words(&self, message: &TraqMessage) -> Result<Vec<MatchedWord>, String>;
    async fn find_matching_stamps(
        &self,
        message: &TraqMessage,
    ) -> Result<Vec<MatchedStamp>, String>;
}

pub struct MatchedWord {
    pub word_uuid: Uuid,
    pub target_user_id: UserId,
    pub word_value: WordValue,
}

pub struct MatchedStamp {
    pub stamp_uuid: Uuid,
    pub target_user_id: UserId,
    pub stamp_name: TraqStampName,
}
