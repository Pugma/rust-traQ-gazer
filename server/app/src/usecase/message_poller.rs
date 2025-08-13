use crate::infra::traq::message_poller_impl::TraqMessagePoller;

pub struct MessagePollerService {
    poller: TraqMessagePoller,
}
impl MessagePollerService {
    pub fn new(poller: TraqMessagePoller) -> Self {
        Self { poller }
    }

    pub async fn start_polling(&self) -> Result<(), String> {
        self.poller.poll_messages().await
    }
}

pub trait MessagePoller {
    async fn poll_messages(&self) -> Result<(), String>;
}
