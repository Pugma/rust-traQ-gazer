use tokio::{spawn, try_join};

use crate::{
    infra::{repo::Repository, traq::message_collector::TraqMessageCollector},
    usecase::{
        message_poller::MessagePollerService, user_synchronizer::UserSynchronizerService,
        word::WordService,
    },
};

pub mod message_poller;
mod message_processor;
mod stamp;
mod user_synchronizer;
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

pub struct BackgroundTasks {
    user_synchronizer: UserSynchronizerService,
    message_poller: MessagePollerService,
}
impl BackgroundTasks {
    pub fn new(repo: Repository, collector: TraqMessageCollector) -> Self {
        Self {
            user_synchronizer: UserSynchronizerService::new(repo.clone()),
            message_poller: MessagePollerService::new(repo, collector, 180),
        }
    }

    pub async fn start(self) -> () {
        let a = spawn(async move { self.user_synchronizer.sync_with_traq().await });
        let b = spawn(async move { self.message_poller.start_polling().await });

        let _ = try_join!(a, b);
    }
}
