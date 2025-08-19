use crate::domain::{traq_stamp::TraqStampId, user::UserId};
use anyhow::Result;
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct StampId(pub i32);

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct StampUuid(pub Uuid);

pub struct NewStamp {
    uuid: StampUuid,
    user_id: UserId,
    traq_stamp_id: TraqStampId,
}

impl NewStamp {
    pub fn new(user_id: UserId, traq_stamp_id: TraqStampId) -> Self {
        Self {
            uuid: StampUuid(Uuid::new_v4()),
            user_id,
            traq_stamp_id,
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
    pub id: StampId,
    pub uuid: StampUuid,
    pub user_id: UserId,
    pub traq_stamp_id: TraqStampId,
    pub excluded_stamp_user_ids: Vec<UserId>,
}

pub trait StampRepository {
    async fn insert_stamp(&self, stamp: NewStamp) -> Result<()>;
    async fn get_all_stamps(&self) -> Result<Vec<Stamp>>;
    async fn find_stamps_by_user_id(&self, user_id: &UserId) -> Result<Vec<Stamp>>;
    async fn delete_stamp(&self, stamp_id: &StampId) -> Result<()>;
}
