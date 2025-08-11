use traq::apis::configuration::Configuration;

pub struct TraqNotificationService {
    config: Configuration,
}

impl TraqNotificationService {
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }
}
