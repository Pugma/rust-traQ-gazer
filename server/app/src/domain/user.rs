use uuid::Uuid;

pub struct UserId(i32);

pub struct User {
    id: UserId,
    display_name: String,
    traq_id: String,
    traq_uuid: Uuid,
    is_bot: bool,
}

pub trait UserRepository {
    async fn sync_with_traq(&self) -> Result<(), String>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<User, String>;
    async fn find_by_traq_uuid(&self, traq_uuid: Uuid) -> Result<User, String>;
}
