use anyhow::Result;
use chrono::{DateTime, SecondsFormat, Utc};
use log::{debug, error, info};
use traq::apis::{configuration::Configuration, message_api::search_messages};

use super::MESSAGE_LIMIT;
use crate::infra::repo::Repository;

pub async fn collect(
    repo: &Repository,
    config: &Configuration,
    checkpoint: &mut DateTime<Utc>,
) -> Result<()> {
    if let Some(token) = config.bearer_access_token.clone() {
        debug!("bot_access_token is Some object");
        if token == *"" {
            error!("bot access token was empty");
            return Ok(());
        };
    }

    let now = Utc::now();

    for page in 0.. {
        let result = search_messages(
            config,
            None,
            Some(checkpoint.to_rfc3339_opts(SecondsFormat::Nanos, true)),
            Some(now.to_rfc3339_opts(SecondsFormat::Nanos, true)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(MESSAGE_LIMIT),
            Some(MESSAGE_LIMIT * page),
            Some("-createdAt"),
        )
        .await;

        let result = if let Ok(result) = result {
            result
        } else {
            error!("Couldn't get messages from traQ!");
            break;
        };

        let hit_messages = result.hits;
        info!("Collected {} messages", hit_messages.len());

        // メッセージを処理
        let repo_clone = repo.clone();
        let messages_clone = hit_messages.clone();
        tokio::spawn(async move {
            // if let Err(e) = process_messages(messages_clone, &repo_clone).await {
            //     error!("Failed to process messages: {}", e);
            // }
        });

        // 全メッセージが取得されたかチェック
        if MESSAGE_LIMIT * (page + 1) >= result.total_hits as i32 {
            if hit_messages.is_empty() {
                *checkpoint = now;
                info!("Updated last_checkpoint = {}", *checkpoint);
                break;
            }

            // 最新メッセージのタイムスタンプを取得
            *checkpoint = hit_messages.last().unwrap().created_at.clone().parse()?;
            info!("Updated last_checkpoint = {}", *checkpoint);
            break;
        }
    }

    repo.record_time(checkpoint.clone()).await?;

    Ok(())
}
