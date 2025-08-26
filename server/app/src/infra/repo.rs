use anyhow::{Ok, Result};
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

use crate::config::repo::DB_CONFIG;

mod polling;
mod stamps;
mod traq_stamp;
mod user;
mod word;

pub mod constant {
    pub const BIND_LIMIT: usize = 65535;
}

#[derive(Clone)]
pub struct Repository {
    pool: Pool<MySql>,
}

impl Repository {
    pub async fn setup() -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .connect_with(DB_CONFIG.options())
            .await?;

        Ok(Self { pool })
    }
}
