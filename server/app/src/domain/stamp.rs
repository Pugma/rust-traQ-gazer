use crate::domain::{traq_stamp::TraqStampId, user::UserId};
use uuid::Uuid;

pub struct StampId(i32);
pub struct StampUuid(Uuid);

pub struct Stamp {
    id: StampId,
    uuid: StampUuid,
    user_id: UserId,
    traq_stamp_id: TraqStampId,
    excluded_stamp_user_ids: Vec<UserId>,
}

pub trait StampRepository {
    async fn register(&self, stamp: Stamp) -> Result<(), String>;
    async fn find_all(&self) -> Result<Vec<Stamp>, String>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Stamp>, String>;
    async fn delete(&self, stamp_id: &StampId) -> Result<(), String>;
}
