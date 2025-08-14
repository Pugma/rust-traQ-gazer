use std::sync::LazyLock;

pub struct TraqConfig {
    pub bot_access_token: String,
}

pub static TRAQ_CONFIG: LazyLock<TraqConfig> = LazyLock::new(|| TraqConfig {
    bot_access_token: std::env::var("TRAQ_BOT_ACCESS_TOKEN")
        .expect("TRAQ_BOT_ACCESS_TOKEN must be set"),
});
