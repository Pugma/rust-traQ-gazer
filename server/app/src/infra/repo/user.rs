use std::cmp::{max, min};

use anyhow::Result;
use sqlx::{MySql, QueryBuilder};
use uuid::Uuid;

use crate::{
    domain::user::{NewUser, User, UserId, UserRepository},
    infra::repo::{Repository, constant::BIND_LIMIT},
};

impl UserRepository for Repository {
    async fn upsert_users(&self, users: Vec<NewUser>) -> Result<()> {
        if users.is_empty() {
            return Ok(());
        }

        let max_rows = max(1, min(users.len(), BIND_LIMIT / 4));
        for chunk in users.chunks(max_rows) {
            let mut qb = QueryBuilder::<MySql>::new(
                r#"
            INSERT INTO
                `users` (`display_name`, `traq_id`, `traq_uuid`, `is_bot`, `is_expired`)
            "#,
            );

            qb.push_values(chunk.iter(), |mut b, u| {
                b.push_bind(&u.display_name);
                b.push_bind(&u.traq_id);
                b.push_bind(&u.traq_uuid);
                b.push_bind(&u.is_bot);
                b.push_bind(&u.is_expired);
            });

            qb.push(
                r#"
                ON DUPLICATE KEY UPDATE
                    `display_name` = VALUES(`display_name`),
                    `traq_id` = VALUES(`traq_id`),
                    `traq_uuid` = VALUES(`traq_uuid`),
                    `is_bot` = VALUES(`is_bot`),
                    `is_expired` = VALUES(`is_expired`)
                "#,
            );

            let q = qb.build();
            q.execute(&self.pool)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to upsert users: {}", e))?;
        }

        Ok(())
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<User> {
        unimplemented!()
    }

    async fn find_by_traq_uuid(&self, traq_uuid: Uuid) -> Result<User> {
        unimplemented!()
    }
}
