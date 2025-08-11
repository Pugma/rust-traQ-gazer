use sqlx::{query, query_as};
use uuid::Uuid;

use crate::domain::{
    user::UserId,
    word::{NewWord, Word, WordId, WordRepository, WordUuid, WordValue},
};

use super::Repository;

impl WordRepository for Repository {
    async fn insert_word(&self, word: NewWord) -> Result<(), String> {
        let result = query!(
            r#"
                INSERT INTO 
                    `words` (`uuid`, `user_id`, `value`, `is_regex`)
                VALUES
                    (?, ?, ?, ?)
            "#,
            word.uuid(),
            word.user_id(),
            word.value(),
            word.is_regex()
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn get_all_words(&self) -> Result<Vec<Word>, String> {
        let rows = query!(
            r#"
                SELECT 
                    `id`, `uuid`, `w`.`user_id`, `value`, `is_regex`,
                    GROUP_CONCAT(`word_excluded_users`.`user_id` SEPARATOR ',') AS `excluded_user_ids`
                FROM 
                    `words` w
                LEFT JOIN
                    `word_excluded_users`
                ON
                    `w`.`id` = `word_excluded_users`.`word_id`
                GROUP BY
                    `w`.`id`
            "#
        )
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(rows) => {
                let words = rows
                    .into_iter()
                    .map(|row| Word {
                        id: WordId(row.id),
                        uuid: WordUuid(Uuid::from_slice(&row.uuid).unwrap()),
                        user_id: UserId(row.user_id),
                        value: WordValue(row.value),
                        is_regex: row.is_regex != 0,
                        excluded_message_user_ids: row
                            .excluded_user_ids
                            .unwrap_or("".to_string())
                            .split(',')
                            .filter_map(|id| id.parse().ok().map(UserId))
                            .collect(),
                    })
                    .collect();
                Ok(words)
            }
            Err(err) => Err(err.to_string()),
        }
    }

    async fn find_words_by_user_id(
        &self,
        user_id: &UserId,
    ) -> std::result::Result<Vec<Word>, String> {
        let rows = query!(
            r#"
                SELECT 
                    `id`, `uuid`, `w`.`user_id`, `value`, `is_regex`
                FROM 
                    `words` w
                LEFT JOIN
                    `word_excluded_users`
                ON
                    `w`.`id` = `word_excluded_users`.`word_id`
                WHERE
                    `w`.`user_id` = ?
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(rows) => {
                let words = rows
                    .into_iter()
                    .map(|row| Word {
                        id: WordId(row.id),
                        uuid: WordUuid(Uuid::from_slice(&row.uuid).unwrap()),
                        user_id: UserId(row.user_id),
                        value: WordValue(row.value),
                        is_regex: row.is_regex != 0,
                        excluded_message_user_ids: Vec::new(),
                    })
                    .collect();
                Ok(words)
            }
            Err(err) => Err(err.to_string()),
        }
    }

    async fn delete_word(&self, word_id: &WordId) -> Result<(), String> {
        let rows = query!(
            r#"
                DELETE FROM `words`
                WHERE
                    `id` = ?
            "#,
            word_id
        )
        .execute(&self.pool)
        .await;

        match rows {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
