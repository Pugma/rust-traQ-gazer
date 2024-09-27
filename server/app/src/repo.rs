use anyhow::{Ok, Result};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySql, Pool,
};

mod words;

pub mod constant {
    pub const BIND_LIMIT: usize = 65535;
}

#[derive(Clone)]
pub struct Repository {
    pub pool: Pool<MySql>,
}

impl Repository {
    pub async fn setup() -> Result<Self> {
        let options = MySqlConnectOptions::new()
            .host("host")
            .port(1111)
            .username("username")
            .password("password")
            .database("database");

        let pool = MySqlPoolOptions::new().connect_with(options).await?;

        Ok(Self { pool })
    }
}
