use anyhow::Result;
use uuid::Uuid;

#[derive(sqlx::Type, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct UserId(pub i64);

pub struct NewUser {
    pub display_name: String,
    pub traq_id: String,
    pub traq_uuid: Uuid,
    pub is_bot: bool,
    pub is_expired: bool,
}

pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub traq_id: String,
    pub traq_uuid: Uuid,
    pub is_bot: bool,
    pub is_expired: bool,
}

pub trait UserRepository {
    async fn upsert_users(&self, users: Vec<NewUser>) -> Result<()>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<User>;
    async fn find_by_traq_id(&self, traq_id: &str) -> Result<User>;
    async fn find_by_traq_uuid(&self, traq_uuid: Uuid) -> Result<User>;
}
