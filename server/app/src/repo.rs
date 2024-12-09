use std::{env, sync::LazyLock};

use anyhow::{Ok, Result};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySql, Pool,
};

mod polling;
mod traq_message;
mod words;

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

struct Config {
    db_database: String,
    db_host: String,
    db_password: String,
    db_port: u16,
    db_username: String,
}

impl Config {
    fn options(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .database(&self.db_database)
            .host(&self.db_host)
            .password(&self.db_password)
            .port(self.db_port)
            .username(&self.db_username)
            .collation("utf8mb4_general_ci")
    }
}

static DB_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    db_database: env::var("DB_DATABASE").expect("Couldn't find DB_DATABASE"),
    db_host: env::var("DB_HOST").expect("Couldn't find DB_HOST"),
    db_password: env::var("DB_PASSWORD").expect("Couldn't find DB_PASSWORD"),
    db_port: env::var("DB_PORT")
        .expect("Couldn't find DB_PORT")
        .parse()
        .expect("Couldn't parse String to u16"),
    db_username: env::var("DB_USERNAME").expect("Couldn't find DB_USERNAME"),
});
