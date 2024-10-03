use anyhow::{Ok, Result};
use sqlx::{query, query_as, types::time::PrimitiveDateTime};

use super::Repository;

struct Polling {
    last: PrimitiveDateTime,
}

impl Repository {
    pub async fn get_time(&self) -> Result<String> {
        let result = query_as!(Polling, "SELECT `last` FROM `polling` WHERE `key`=1")
            .fetch_one(&self.pool)
            .await?;

        Ok(result.last.to_string())
    }

    pub async fn record_time(&self, checkpoint: String) -> Result<()> {
        query!("INSERT INTO `polling`(`key`, `last`) VALUES(1, ?) ON DUPLICATE KEY UPDATE `last`=VALUES(`last`)", checkpoint,).execute(&self.pool).await?;

        Ok(())
    }
}