use crate::domain::{traq_stamp::TraqStampName, user::UserId, word::WordValue};
use uuid::Uuid;

use anyhow::Result;

#[derive(Clone)]
pub struct TraqMessageUuid(pub Uuid);

impl TraqMessageUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

pub struct TraqMessage {
    uuid: TraqMessageUuid,
    author_user_id: UserId,
    content: String,
    stamps: Vec<TraqMessageStamp>,
}

impl TraqMessage {
    pub fn new(
        uuid: TraqMessageUuid,
        author_user_id: UserId,
        content: String,
        stamps: Vec<TraqMessageStamp>,
    ) -> Self {
        Self {
            uuid,
            author_user_id,
            content,
            stamps,
        }
    }

    pub fn uuid(&self) -> &TraqMessageUuid {
        &self.uuid
    }

    pub fn author_user_id(&self) -> &UserId {
        &self.author_user_id
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

pub struct TraqMessageStamp {
    stamp_uuid: Uuid,
    user_id: UserId,
}

impl TraqMessageStamp {
    pub fn new(stamp_uuid: Uuid, user_id: UserId) -> Self {
        Self {
            stamp_uuid,
            user_id,
        }
    }
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
