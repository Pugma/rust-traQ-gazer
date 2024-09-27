use anyhow::{Ok, Result};
use sqlx::{query, query_as, Execute, MySql, QueryBuilder};

use openapi::models::{ExcludedUsers, MyWords, Word, Words};
use uuid::Uuid;

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

    pub async fn get_my_word(&self, _trap_id: String) -> Result<MyWords> {
        // let result = query_as!(
        //     MyWord,
        //     "SELECT
        //         `word`,
        //         `word_uuid` AS `id`,
        //         `register_time` AS `time`,
        //         `excluded_users`.`trap_id` AS `excluded_users`
        //     FROM `words` JOIN `excluded_users` ON `words`.`word_id` = `excluded_users`.`word_id`
        //     WHERE `words`.`trap_id`=?",
        //     trap_id
        // )
        // .fetch_all(&self.pool)
        // .await?;

        // Ok(result.into())

        unimplemented!()
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
        query!("DELETE FROM `excluded_users` WHERE `word_id` = ?", word_id,)
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
