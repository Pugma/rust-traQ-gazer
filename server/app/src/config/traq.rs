use std::sync::LazyLock;

use traq::apis::configuration::Configuration;

pub struct TraqConfig {
    bot_access_token: String,
}
impl TraqConfig {
    pub fn options(&self) -> Configuration {
        Configuration {
            bearer_access_token: Some(self.bot_access_token.clone()),
            ..Default::default()
        }
    }
}

pub static TRAQ_CONFIG: LazyLock<TraqConfig> = LazyLock::new(|| TraqConfig {
    bot_access_token: std::env::var("TRAQ_BOT_ACCESS_TOKEN")
        .expect("TRAQ_BOT_ACCESS_TOKEN must be set"),
});
