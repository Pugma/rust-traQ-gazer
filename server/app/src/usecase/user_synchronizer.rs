use std::sync::Arc;

use anyhow::Result;
use log::error;
use tokio_cron_scheduler::JobBuilder;

use crate::{
    domain::user::NewUser,
    infra::{repo::Repository, traq::user_fetcher::TraqUserFetcher},
};

pub struct UserSynchronizerService {
    repo: Repository,
    user_fetcher: Arc<TraqUserFetcher>,
}

impl UserSynchronizerService {
    pub fn new(repo: Repository, user_fetcher: Arc<TraqUserFetcher>) -> Self {
        Self { repo, user_fetcher }
    }

    pub async fn sync_with_traq(&self) -> Result<()> {
        let uf = self.user_fetcher.clone();
        let job = JobBuilder::new()
            .with_timezone(chrono_tz::Asia::Tokyo)
            .with_cron_job_type()
            .with_schedule("0 0 4 * * * *")?
            .with_run_async(Box::new(move |_uuid, _l| {
                let f = uf.clone();
                Box::pin(async move {
                    let users = match f.fetch_users().await {
                        Ok(users) => users,
                        Err(e) => {
                            error!("Failed to fetch users: {}", e);
                            return;
                        }
                    };
                })
            }))
            .build()?;

        Ok(())
    }
}

pub trait UserFetcher {
    async fn fetch_users(&self) -> Result<Vec<NewUser>>;
}
