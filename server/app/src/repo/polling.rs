use anyhow::{Ok, Result};
use chrono::Utc;
use sqlx::{
    query, query_as,
    types::chrono::{DateTime, NaiveDateTime},
};

use super::Repository;

struct Polling {
    last: NaiveDateTime,
}

impl Repository {
    pub async fn get_time(&self) -> Result<DateTime<Utc>> {
        let result = query_as!(Polling, "SELECT `last` FROM `polling` WHERE `key`=1")
            .fetch_one(&self.pool)
            .await?;

        Ok(result.last.and_utc())
    }

    pub async fn record_time(&self, checkpoint: DateTime<Utc>) -> Result<()> {
        let checkpoint = checkpoint.naive_utc();
        query!("INSERT INTO `polling`(`key`, `last`) VALUES(1, ?) ON DUPLICATE KEY UPDATE `last`=VALUES(`last`)", checkpoint).execute(&self.pool).await?;

        Ok(())
    }
}
