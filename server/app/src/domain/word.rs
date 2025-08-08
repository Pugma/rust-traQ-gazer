use crate::domain::user::UserId;
use uuid::Uuid;

pub struct WordId(i32);
pub struct WordUuid(Uuid);
pub struct WordValue(String);

impl WordValue {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("word cannot be empty".to_string());
        }
        if value.chars().count() > 50 {
            return Err("word must be 50 characters or less".to_string());
        }
        Ok(WordValue(value))
    }
}

pub struct Word {
    id: WordId,
    uuid: WordUuid,
    user_id: UserId,
    value: WordValue,
    is_regex: bool,
    excluded_message_user_ids: Vec<UserId>,
}

pub trait WordRepository {
    async fn register(&self, word: Word) -> Result<(), String>;
    async fn find_all(&self) -> Result<Vec<Word>, String>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Word>, String>;
    async fn delete(&self, word_id: &WordId) -> Result<(), String>;
}
