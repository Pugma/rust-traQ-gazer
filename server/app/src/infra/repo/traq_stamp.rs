use std::cmp::{max, min};

use anyhow::Result;
use sqlx::{QueryBuilder, query_as};
use uuid::Uuid;

use crate::{
    domain::traq_stamp::{
        TraqStamp, TraqStampId, TraqStampName, TraqStampRepository, TraqStampUuid,
    },
    infra::repo::{Repository, constant::BIND_LIMIT},
};

struct TraqStampRow {
    id: i64,
    name: String,
    traq_uuid: Vec<u8>,
}

impl TraqStampRepository for Repository {
    async fn upsert_stamps(&self, stamps: Vec<TraqStamp>) -> Result<()> {
        if stamps.is_empty() {
            return Ok(());
        }

        let max_rows = max(1, min(stamps.len(), BIND_LIMIT / 3));
        for chunk in stamps.chunks(max_rows) {
            let mut qb = QueryBuilder::new(
                r#"
                    INSERT INTO
                        `traq_stamps` (`name`, `traq_uuid`)
                "#,
            );

            qb.push_values(chunk.iter(), |mut b, stamp| {
                b.push_bind(&stamp.name().0);
                b.push_bind(stamp.uuid().0.as_bytes().to_vec());
            });

            qb.push(
                r#"
                    ON DUPLICATE KEY UPDATE
                        `name` = VALUES(`name`),
                        `traq_uuid` = VALUES(`traq_uuid`)
                "#,
            );

            let q = qb.build();
            q.execute(&self.pool)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to upsert stamps: {}", e))?;
        }

        Ok(())
    }

    async fn find_stamp_by_name(&self, name: &str) -> Result<Option<TraqStamp>> {
        let row = query_as!(
            TraqStampRow,
            r#"
                SELECT
                    `id`, `name`, `traq_uuid`
                FROM
                    `traq_stamps`
                WHERE
                    `name` = ?
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            TraqStamp::new(
                TraqStampId(r.id),
                TraqStampName(r.name),
                TraqStampUuid(Uuid::from_slice(&r.traq_uuid).unwrap()),
            )
        }))
    }

    async fn find_stamp_by_uuid(&self, uuid: &Uuid) -> Result<Option<TraqStamp>> {
        let row = query_as!(
            TraqStampRow,
            r#"
                SELECT
                    `id`, `name`, `traq_uuid`
                FROM
                    `traq_stamps`
                WHERE
                    `traq_uuid` = ?
            "#,
            uuid.as_bytes().to_vec()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            TraqStamp::new(
                TraqStampId(r.id),
                TraqStampName(r.name),
                TraqStampUuid(Uuid::from_slice(&r.traq_uuid).unwrap()),
            )
        }))
    }
}
