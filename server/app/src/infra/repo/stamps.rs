use anyhow::Result;
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::{
        stamp::{NewStamp, Stamp, StampId, StampRepository, StampUuid},
        traq_stamp::TraqStampId,
        user::UserId,
    },
    infra::repo::Repository,
};

impl StampRepository for Repository {
    async fn register(&self, stamp: NewStamp) -> Result<()> {
        query!(
            r#"
                INSERT INTO
                    `stamp_subscriptions` (`uuid`, `user_id`, `stamp_id`)
                VALUES
                    (?, ?, ?)
            "#,
            stamp.uuid(),
            stamp.user_id(),
            stamp.traq_stamp_id()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Stamp>> {
        let rows = query!(
            r#"
                SELECT
                    `id`, `uuid`, `ss`.`user_id`, `stamp_id`,
                    GROUP_CONCAT(`seu`.`user_id` SEPARATOR ',') AS `excluded_user_ids`
                FROM
                    `stamp_subscriptions` AS `ss`
                LEFT JOIN
                    `stamp_excluded_users` AS `seu`
                ON
                    `seu`.`subs_id` = `ss`.`id`
                GROUP BY
                    `ss`.`id`
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Stamp {
                id: StampId(row.id),
                uuid: StampUuid(Uuid::from_slice(&row.uuid).unwrap()),
                user_id: UserId(row.user_id),
                traq_stamp_id: TraqStampId(row.stamp_id),
                excluded_stamp_user_ids: row
                    .excluded_user_ids
                    .unwrap()
                    .split(',')
                    .filter_map(|id| id.parse().ok().map(UserId))
                    .collect(),
            })
            .collect())
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Stamp>> {
        let rows = query!(
            r#"
                SELECT
                    `id`, `uuid`, `ss`.`user_id`, `stamp_id`,
                    GROUP_CONCAT(`seu`.`user_id` SEPARATOR ',') AS `excluded_user_ids`
                FROM
                    `stamp_subscriptions` AS `ss`
                LEFT JOIN
                    `stamp_excluded_users` AS `seu`
                ON
                    `seu`.`subs_id` = `ss`.`id`
                WHERE
                    `ss`.`user_id` = ?
                GROUP BY
                    `ss`.`id`
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Stamp {
                id: StampId(row.id),
                uuid: StampUuid(Uuid::from_slice(&row.uuid).unwrap()),
                user_id: UserId(row.user_id),
                traq_stamp_id: TraqStampId(row.stamp_id),
                excluded_stamp_user_ids: row
                    .excluded_user_ids
                    .unwrap()
                    .split(',')
                    .filter_map(|id| id.parse().ok().map(UserId))
                    .collect(),
            })
            .collect())
    }

    async fn delete(&self, stamp_id: &StampId) -> Result<()> {
        query!(
            r#"
                DELETE FROM `stamp_subscriptions`
                WHERE
                    `id` = ?
            "#,
            stamp_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
