use crate::infra::traq::message_poller_impl::TraqMessagePoller;

pub struct MessagePollerService {
    poller: TraqMessagePoller,
    polling_interval_sec: u64,
}
impl MessagePollerService {
    pub fn new(poller: TraqMessagePoller, polling_interval_sec: u64) -> Self {
        Self {
            poller,
            polling_interval_sec,
        }
    }

    pub async fn start_polling(&self) -> Result<(), String> {
        self.poller.poll_messages(self.polling_interval_sec).await
    }
}

pub trait MessagePoller {
    async fn poll_messages(&self, polling_interval: u64) -> Result<(), String>;
}
