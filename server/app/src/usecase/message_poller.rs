use crate::{
    domain::{
        notification::NotificationService, traq_message::TraqMessage, user::UserRepository,
        word::WordRepository,
    },
    infra::{repo::Repository, traq::message_collector::TraqMessageCollector},
    usecase::message_processor::MessageProcessor,
};
use chrono::{DateTime, Utc};
use log::{error, info};
use std::time::Duration;
use tokio::time;

pub struct MessagePollerService<W, N, U>
where
    W: WordRepository,
    N: NotificationService,
    U: UserRepository,
{
    repo: Repository,
    collector: TraqMessageCollector<U>,
    processor: MessageProcessor<W, N, U>,
    polling_interval_sec: u64,
}

impl<W, N, U> MessagePollerService<W, N, U>
where
    W: WordRepository + Send + Sync,
    N: NotificationService + Send + Sync,
    U: UserRepository + Send + Sync,
{
    pub fn new(
        repo: Repository,
        collector: TraqMessageCollector<U>,
        processor: MessageProcessor<W, N, U>,
        polling_interval_sec: u64,
    ) -> Self {
        Self {
            repo,
            collector,
            processor,
            polling_interval_sec,
        }
    }

    pub async fn start_polling(&self) -> Result<(), String> {
        let mut interval = time::interval(Duration::from_secs(self.polling_interval_sec));

        let mut last_checkpoint = match self.repo.get_time().await {
            Ok(point) => point,
            Err(_) => {
                error!("Couldn't get last checkpoint!");
                Utc::now()
            }
        };

        loop {
            interval.tick().await;

            info!("start polling ...");
            let messages = self
                .collector
                .collect_messages(&mut last_checkpoint)
                .await?;

            for message in messages {
                if let Err(e) = self.processor.process_message(&message).await {
                    log::error!("Failed to process message: {}", e);
                }
            }

            if let Err(e) = self.repo.record_time(last_checkpoint).await {
                log::error!("Failed to record time: {}", e);
            }
        }
    }
}

pub trait MessagePoller {
    async fn collect_messages(
        &self,
        last_checkpoint: &mut DateTime<Utc>,
    ) -> Result<Vec<TraqMessage>, String>;
}
