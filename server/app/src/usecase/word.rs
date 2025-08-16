use anyhow::Result;

use crate::{
    domain::{
        user::UserId,
        word::{NewWord, Word, WordId, WordRepository},
    },
    infra::repo::Repository,
};

pub struct WordService {
    repo: Repository,
}

impl WordService {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub async fn register_word(
        &self,
        user_id: UserId,
        value: String,
        is_regex: bool,
    ) -> Result<()> {
        let word = NewWord::new(user_id, value, is_regex, vec![])?;
        self.repo.insert_word(word).await
    }

    pub async fn get_all_words(&self) -> Result<Vec<Word>> {
        self.repo.get_all_words().await
    }

    pub async fn get_personal_words(&self, user_id: UserId) -> Result<Vec<Word>> {
        self.repo.find_words_by_user_id(&user_id).await
    }

    pub async fn delete_word(&self, word_id: WordId) -> Result<()> {
        self.repo.delete_word(&word_id).await?;

        Ok(())
    }
}
