pub mod message;
use std::{env, sync::LazyLock};

pub const MESSAGE_LIMIT: i32 = 100;
pub static ACCESS_TOKEN: LazyLock<String> =
    LazyLock::new(|| env::var("BOT_ACCESS_TOKEN").expect("msg"));
