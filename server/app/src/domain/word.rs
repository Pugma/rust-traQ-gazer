use crate::domain::user::UserId;
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct WordId(pub i64);

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct WordUuid(pub Uuid);

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct WordValue(pub String);

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

    pub fn uuid(&self) -> &WordUuid {
        &self.uuid
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn value(&self) -> &WordValue {
        &self.value
    }
    pub fn is_regex(&self) -> bool {
        self.is_regex
    }
}

pub struct Word {
    pub id: WordId,
    pub uuid: WordUuid,
    pub user_id: UserId,
    pub value: WordValue,
    pub is_regex: bool,
    pub excluded_message_user_ids: Vec<UserId>,
}

pub trait WordRepository {
    async fn insert_word(&self, word: NewWord) -> Result<(), String>;
    async fn get_all_words(&self) -> Result<Vec<Word>, String>;
    async fn find_words_by_user_id(&self, user_id: &UserId) -> Result<Vec<Word>, String>;
    async fn delete_word(&self, word_id: &WordId) -> Result<(), String>;
}
