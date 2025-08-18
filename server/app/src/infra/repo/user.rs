use std::cmp::{max, min};

use anyhow::{Ok, Result};
use sqlx::{MySql, QueryBuilder, query_as};
use uuid::Uuid;

use crate::{
    domain::user::{NewUser, User, UserId, UserRepository},
    infra::repo::{Repository, constant::BIND_LIMIT},
};

struct UserRow {
    id: i64,
    display_name: String,
    traq_id: String,
    traq_uuid: Vec<u8>,
    is_bot: i8,
    is_expired: i8,
}

impl TryFrom<UserRow> for User {
    type Error = anyhow::Error;

    fn try_from(r: UserRow) -> Result<Self> {
        let traq_uuid = Uuid::from_slice(&r.traq_uuid)
            .map_err(|e| anyhow::anyhow!("invalid traq_uuid from DB: {}", e))?;
        Ok(User {
            id: UserId(r.id),
            display_name: r.display_name,
            traq_id: r.traq_id,
            traq_uuid,
            is_bot: r.is_bot != 0,
            is_expired: r.is_expired != 0,
        })
    }
}

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
        let row = query_as!(
            UserRow,
            r#"
                SELECT
                    `id`, `display_name`, `traq_id`,
                    `traq_uuid`, `is_bot`, `is_expired`
                FROM
                    `users`
                WHERE
                    `id` = ?
            "#,
            user_id.0
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User::try_from(row)?)
    }

    async fn find_by_traq_id(&self, traq_id: &str) -> Result<User> {
        let row = query_as!(
            UserRow,
            r#"
                SELECT
                    `id`, `display_name`, `traq_id`,
                    `traq_uuid`, `is_bot`, `is_expired`
                FROM
                    `users`
                WHERE
                    `traq_id` = ?
            "#,
            traq_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User::try_from(row)?)
    }

    async fn find_by_traq_uuid(&self, traq_uuid: Uuid) -> Result<User> {
        let row = query_as!(
            UserRow,
            r#"
                SELECT
                    `id`, `display_name`, `traq_id`,
                    `traq_uuid`, `is_bot`, `is_expired`
                FROM
                    `users`
                WHERE
                    `traq_uuid` = ?
            "#,
            traq_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User::try_from(row)?)
    }
}
