use anyhow::Result;
use axum::async_trait;
use chrono::{DateTime, SecondsFormat, Utc};
use log::{debug, error, info};
use traq::{
    apis::{
        configuration::Configuration, message_api::search_messages, user_api::post_direct_message,
    },
    models::{Message, PostMessageRequest},
};
use uuid::Uuid;

use super::{CONFIG, MESSAGE_LIMIT};
use crate::repo::Repository;

pub(super) async fn collect(
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

        let m = hit_messages.clone();
        let r = repo.clone();
        tokio::spawn(async move {
            let _ = process(m, &r);
        });

        // check whether all messages are retrieved
        if MESSAGE_LIMIT * (page + 1) >= result.total_hits as i32 {
            if hit_messages.is_empty() {
                *checkpoint = now;
                info!("Updated last_checkpoint = {}", *checkpoint);
                break;
            }

            // get the timestamp from the latest message
            *checkpoint = hit_messages.last().unwrap().created_at.clone().parse()?;
            info!("Updated last_checkpoint = {}", *checkpoint);
            break;
        }
    }

    (*repo).record_time(checkpoint.clone()).await?;

    Ok(())
}

async fn process(messages: Vec<Message>, repo: &Repository) -> Result<()> {
    let messages = convert(messages);

    let mut notifies = Vec::<Box<dyn Notify>>::new();

    for message in messages {
        notifies.push(message.match_word(repo).await);
        notifies.push(message.match_stamp(repo).await);
    }

    // These Vectors no longer need to be edited
    let ns: Vec<Box<dyn Notify>> = notifies;

    info!("Sending {} DMs...", ns.len());
    for n in ns {
        let _ = n.send_dm();
    }
    info!("Correctly finished sending DMs!");

    Ok(())
}

pub struct NotificationMessage {
    pub message_uuid: Uuid,
    user_id: Uuid,
    content: String,
    stamps: Vec<Stamp>,
}

pub struct Stamp {
    stamp_id: Uuid,
    user_id: Uuid,
}

pub struct WordNotify {
    pub words: Vec<String>,
    pub target_traq_uuid: Uuid,
    pub message_uuid: Uuid,
}

pub struct StampNotify {
    pub stamps: Vec<String>,
    pub target_traq_uuid: Uuid,
    pub message_uuid: Uuid,
}

fn convert(messages: Vec<Message>) -> Vec<NotificationMessage> {
    let mut a = Vec::<NotificationMessage>::new();

    for i in messages {
        let mut c = Vec::<Stamp>::new();
        for j in i.stamps {
            c.push(Stamp {
                user_id: j.user_id,
                stamp_id: j.stamp_id,
            });
        }

        a.push(NotificationMessage {
            message_uuid: i.id,
            user_id: i.user_id,
            content: i.content,
            stamps: c,
        });
    }

    a
}

impl NotificationMessage {
    async fn match_word(&self, repo: &Repository) -> Box<dyn Notify> {
        let _ = repo.a(self).await;
        unimplemented!()
    }

    async fn match_stamp(&self, repo: &Repository) -> Box<dyn Notify> {
        let _ = repo.b(self).await;
        unimplemented!()
    }
}

#[async_trait]
trait Notify {
    async fn send_dm(&self) -> Result<()>;
}

#[async_trait]
impl Notify for WordNotify {
    async fn send_dm(&self) -> Result<()> {
        let _a = post_direct_message(
            &CONFIG,
            &self.target_traq_uuid.to_string(),
            Some(PostMessageRequest {
                content: format!("{:?}\n{}", self.words, self.message_uuid),
                embed: None,
            }),
        )
        .await;

        Ok(())
    }
}

#[async_trait]
impl Notify for StampNotify {
    async fn send_dm(&self) -> Result<()> {
        let _a = post_direct_message(
            &CONFIG,
            &self.target_traq_uuid.to_string(),
            Some(PostMessageRequest {
                content: format!("{:?}\n{}", self.stamps, self.message_uuid),
                embed: None,
            }),
        )
        .await;

        let _ = match _a {
            Ok(_) => return Ok(()),
            Err(_) => {
                error!("");
                return Ok(());
            }
        };
    }
}
