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
        message_poller::MessagePollerService, user_synchronizer::UserSynchronizerService,
        word::WordService,
    },
};

pub mod message_poller;
pub mod message_processor;
mod stamp;
pub mod user_synchronizer;
mod word;

pub struct UseCase {
    pub word: WordService,
}

impl UseCase {
    pub fn new(repo: Repository) -> Self {
        Self {
            word: WordService::new(repo.clone()),
        }
    }
}

use crate::{
    config::traq::TRAQ_CONFIG, infra::traq::notification::TraqNotificationService,
    usecase::message_processor::MessageProcessor,
};

pub struct BackgroundTasks {
    user_synchronizer: UserSynchronizerService,
    message_poller:
        MessagePollerService<Repository, TraqNotificationService, Repository>,
}

impl BackgroundTasks {
    pub fn new(
        repo: Repository,
        message_collector: TraqMessageCollector<Repository>,
        user_fetcher: TraqUserFetcher,
    ) -> Self {
        let processor = MessageProcessor::new(
            Arc::new(repo.clone()),
            Arc::new(TraqNotificationService::new(TRAQ_CONFIG.options())),
            Arc::new(repo.clone()),
        );
        Self {
            user_synchronizer: UserSynchronizerService::new(
                Arc::new(repo.clone()),
                Arc::new(user_fetcher),
            ),
            message_poller: MessagePollerService::new(
                repo.clone(),
                message_collector,
                processor,
                180,
            ),
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
