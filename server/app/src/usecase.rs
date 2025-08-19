use std::sync::Arc;

use anyhow::Result;
use tokio::{join, spawn};
use tokio_cron_scheduler::JobScheduler;

use crate::{
    infra::{
        repo::Repository,
        traq::{message_collector::TraqMessageCollector, user_fetcher::TraqUserFetcher},
    },
    usecase::{
        message_poller::MessagePollerService, stamp::StampService,
        user_synchronizer::UserSynchronizerService, word::WordService,
    },
};

pub mod message_poller;
mod message_processor;
pub mod stamp;
pub mod user_synchronizer;
pub mod word;

pub struct UseCase {
    pub word: WordService,
    pub stamp: StampService,
}

impl UseCase {
    pub fn new(repo: Repository) -> Self {
        Self {
            word: WordService::new(repo.clone()),
            stamp: StampService::new(repo),
        }
    }
}

pub struct BackgroundTasks {
    user_synchronizer: UserSynchronizerService,
    message_poller: MessagePollerService,
}
impl BackgroundTasks {
    pub fn new(
        repo: Repository,
        message_collector: TraqMessageCollector,
        user_fetcher: TraqUserFetcher,
    ) -> Self {
        Self {
            user_synchronizer: UserSynchronizerService::new(
                Arc::new(repo.clone()),
                Arc::new(user_fetcher),
            ),
            message_poller: MessagePollerService::new(repo, message_collector, 180),
        }
    }

    pub async fn start(self) -> Result<()> {
        let scheduler = JobScheduler::new().await?;
        let sync = self.user_synchronizer.sync_with_traq().await?;
        let b = spawn(async move { self.message_poller.start_polling().await });

        scheduler.add(sync).await?;
        scheduler.start().await?;

        let _ = join!(b);

        Ok(())
    }
}
