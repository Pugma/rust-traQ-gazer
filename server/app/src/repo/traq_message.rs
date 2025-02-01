use anyhow::Result;
use sqlx::query;
use uuid::Uuid;

use super::Repository;
use crate::traq::message::{NotificationMessage, StampNotify, WordNotify};

impl Repository {
    pub async fn a(&self, a: &NotificationMessage) -> Result<WordNotify> {
        let matched_words = query!(
            "SELECT
                `word`,
                `traq_uuid`
            FROM
                `words`
            JOIN
                `word_excluded_users`
            ON
                `words`.`word_id` = `word_excluded_users`.`word_id`
            JOIN
                `users`
            ON
                `words`.`trap_id` = `users`.`trap_id`
            "
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(WordNotify {
            words: matched_words.iter().map(|a| a.word.clone()).collect(),
            target_traq_uuid: Uuid::parse_str(&(matched_words[0].traq_uuid))?,
            message_uuid: a.message_uuid,
        })
    }

    pub async fn b(&self, a: &NotificationMessage) -> Result<StampNotify> {
        let _aa = query!(
            "SELECT
                `word`,
                `traq_uuid`
            FROM
                `words`
            JOIN
                `word_excluded_users`
            ON
                `words`.`word_id` = `word_excluded_users`.`word_id`
            JOIN
                `users`
            ON
                `words`.`trap_id` = `users`.`trap_id`
            "
        );

        Ok(StampNotify {
            stamps: vec!["".to_string()],
            target_traq_uuid: Uuid::new_v4(),
            message_uuid: a.message_uuid,
        })
    }
}
