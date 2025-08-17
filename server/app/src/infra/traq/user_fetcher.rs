use anyhow::Result;
use log::info;
use traq::apis::user_api;

use crate::{
    config::traq::TRAQ_CONFIG, domain::user::NewUser, usecase::user_synchronizer::UserFetcher,
};

pub struct TraqUserFetcher {}

impl TraqUserFetcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserFetcher for TraqUserFetcher {
    async fn fetch_users(&self) -> Result<Vec<NewUser>> {
        let cfg = TRAQ_CONFIG.options();

        info!("Fetching users from traQ...");
        let traq_users = user_api::get_users(&cfg, Some(true), None).await?;
        let a: Vec<NewUser> = traq_users
            .into_iter()
            .map(|u| NewUser {
                display_name: u.display_name,
                traq_id: u.name,
                traq_uuid: u.id,
                is_bot: u.bot,
            })
            .collect();

        Ok(a)
    }
}
