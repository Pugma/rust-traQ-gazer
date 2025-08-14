use crate::{
    domain::traq_message::TraqMessage,
    infra::{repo::Repository, traq::message_collector::TraqMessageCollector},
};
use chrono::{DateTime, Utc};
use log::{error, info};
use std::time::Duration;
use tokio::time;

pub struct MessagePollerService {
    repo: Repository,
    collector: TraqMessageCollector,
    polling_interval_sec: u64,
}
impl MessagePollerService {
    pub fn new(
        repo: Repository,
        collector: TraqMessageCollector,
        polling_interval_sec: u64,
    ) -> Self {
        Self {
            repo,
            collector,
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
            self.collector
                .collect_messages(&mut last_checkpoint)
                .await?;

            self.repo.record_time(last_checkpoint).await;
        }
    }
}

pub trait MessagePoller {
    async fn collect_messages(
        &self,
        last_checkpoint: &mut DateTime<Utc>,
    ) -> Result<Vec<TraqMessage>, String>;
}
