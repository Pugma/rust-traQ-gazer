use crate::domain::{traq_stamp::TraqStampId, user::UserId};
use uuid::Uuid;

pub struct StampId(i32);
pub struct StampUuid(Uuid);

pub struct NewStamp {
    uuid: StampUuid,
    user_id: UserId,
    traq_stamp_id: TraqStampId,
    excluded_stamp_user_ids: Vec<UserId>,
}
impl NewStamp {
    pub fn new(
        user_id: UserId,
        traq_stamp_id: TraqStampId,
        excluded_stamp_user_ids: Vec<UserId>,
    ) -> Self {
        NewStamp {
            uuid: StampUuid(Uuid::new_v4()),
            user_id,
            traq_stamp_id,
            excluded_stamp_user_ids,
        }
    }

    pub fn uuid(&self) -> &StampUuid {
        &self.uuid
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn traq_stamp_id(&self) -> &TraqStampId {
        &self.traq_stamp_id
    }
}

pub struct Stamp {
    id: StampId,
    uuid: StampUuid,
    user_id: UserId,
    traq_stamp_id: TraqStampId,
    excluded_stamp_user_ids: Vec<UserId>,
}

pub trait StampRepository {
    async fn register(&self, stamp: NewStamp) -> Result<(), String>;
    async fn find_all(&self) -> Result<Vec<Stamp>, String>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Stamp>, String>;
    async fn delete(&self, stamp_id: &StampId) -> Result<(), String>;
}
