use anyhow::{Ok, Result};
use openapi::models::{ExcludedUser, ExcludedUsers, MyWord, MyWords, Word, Words};
use sqlx::{query, query_as, types::uuid::Uuid, Execute, MySql, QueryBuilder};
use std::collections::HashMap;

use super::{constant::BIND_LIMIT, Repository};

impl Repository {
    pub async fn register(&self, trap_id: String, word: String) -> Result<()> {
        query!(
            "
            INSERT INTO `words` (`word_uuid`, `trap_id`, `word`)
            VALUES (?, ?, ?)
            ",
            Uuid::new_v4(),
            trap_id,
            word
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, trap_id: String, word_id: Uuid) -> Result<()> {
        query!(
            "DELETE FROM `words` WHERE `trap_id` = ? AND `word_id` = ?",
            trap_id,
            word_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Words> {
        let result = query_as!(Word, "SELECT `trap_id`, `word` FROM `words`")
            .fetch_all(&self.pool)
            .await?;

        Ok(result.into())
    }

    pub async fn get_by_word(&self, word: String) -> Result<Words> {
        let result = query_as!(
            Word,
            "SELECT `trap_id`, `word` FROM `words` WHERE `word`=?",
            word
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result.into())
    }

    pub async fn get_my_word(&self, trap_id: String) -> Result<MyWords> {
        let rows = query!(
            "SELECT
                `word`,
                `word_uuid` AS `id`,
                `register_time` AS `time`,
                `word_excluded_users`.`trap_id` AS `excluded_users`
            FROM
                `words`
            JOIN
                `word_excluded_users`
            ON
                `words`.`word_id` = `word_excluded_users`.`word_id`
            WHERE
                `words`.`trap_id`=?",
            trap_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut a: HashMap<String, MyWord> = HashMap::new();
        for row in rows {
            let entry = a.entry(row.word.clone()).or_insert(MyWord {
                word: row.word,
                id: Uuid::from_slice(&row.id)?,
                time: row.time.unwrap().and_utc(),
                excluded_users: Vec::<ExcludedUser>::new().into(),
            });

            entry.excluded_users.push(ExcludedUser {
                trap_id: row.excluded_users,
            });
        }

        let my_words: Vec<MyWord> = a.into_values().collect();

        Ok(my_words.into())
    }

    pub async fn get_by_user(&self, trap_id: String) -> Result<Words> {
        let result = query_as!(
            Word,
            "SELECT `trap_id`, `word` FROM `words` WHERE `trap_id`=?",
            trap_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result.into())
    }

    pub async fn edit_excluded_users(
        &self,
        word_id: Uuid,
        excluded_users: ExcludedUsers,
    ) -> Result<()> {
        query!(
            "DELETE FROM `word_excluded_users` WHERE `word_id` = ?",
            word_id,
        )
        .execute(&self.pool)
        .await?;

        let mut query_builder =
            QueryBuilder::<MySql>::new("INSERT INTO `excluded_users`(`word_id`, `trap_id`) ");

        query_builder.push_values(&excluded_users[0..=BIND_LIMIT / 2], |mut b, users| {
            b.push_bind(word_id).push_bind(users.trap_id.clone());
        });

        let query = query_builder.build();
        query.sql();

        Ok(())
    }
}
