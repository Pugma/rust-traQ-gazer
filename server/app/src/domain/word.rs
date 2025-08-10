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

pub struct NewWord {
    uuid: WordUuid,
    user_id: UserId,
    value: WordValue,
    is_regex: bool,
    excluded_message_user_ids: Vec<UserId>,
}
impl NewWord {
    pub fn new(
        user_id: UserId,
        value: String,
        is_regex: bool,
        excluded_message_user_ids: Vec<UserId>,
    ) -> Result<Self, String> {
        Ok(NewWord {
            uuid: WordUuid(Uuid::new_v4()),
            user_id,
            value: WordValue::new(value)?,
            is_regex,
            excluded_message_user_ids,
        })
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
    async fn insert_word(&self, word: NewWord) -> Result<(), String>;
    async fn get_all_words(&self) -> Result<Vec<Word>, String>;
    async fn find_words_by_user_id(&self, user_id: &UserId) -> Result<Vec<Word>, String>;
    async fn delete_word(&self, word_id: &WordId) -> Result<(), String>;
}
