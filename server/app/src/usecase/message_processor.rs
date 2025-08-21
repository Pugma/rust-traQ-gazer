use anyhow::Result;
use futures::stream::{self, StreamExt};
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{
    notification::{NotificationService, WordNotification},
    traq_message::TraqMessage,
    user::UserRepository,
    word::WordRepository,
};

pub struct MessageProcessor<W, N, U>
where
    W: WordRepository,
    N: NotificationService,
    U: UserRepository,
{
    word_repo: Arc<W>,
    notification_service: Arc<N>,
    user_repo: Arc<U>,
}

impl<W, N, U> MessageProcessor<W, N, U>
where
    W: WordRepository + Send + Sync,
    N: NotificationService + Send + Sync,
    U: UserRepository + Send + Sync,
{
    pub fn new(word_repo: Arc<W>, notification_service: Arc<N>, user_repo: Arc<U>) -> Self {
        Self {
            word_repo,
            notification_service,
            user_repo,
        }
    }

    pub async fn process_message(&self, message: &TraqMessage) -> Result<()> {
        let words = self.word_repo.get_all_words().await?;
        let user_words = words.into_iter().fold(HashMap::new(), |mut acc, word| {
            acc.entry(word.user_id).or_insert_with(Vec::new).push(word);
            acc
        });

        stream::iter(user_words)
            .for_each_concurrent(None, |(user_id, words)| async move {
                let matched_words = words
                    .into_iter()
                    .filter(|word| {
                        if word
                            .excluded_message_user_ids
                            .iter()
                            .any(|excluded_id| *excluded_id == *message.author_user_id())
                        {
                            return false;
                        }
                        if word.is_regex {
                            // NOTE: invalid regex is not considered
                            let re = match Regex::new(&word.value.0) {
                                Ok(re) => re,
                                Err(e) => {
                                    log::error!("Invalid regex: {}", e);
                                    return false;
                                }
                            };
                            re.is_match(message.content())
                        } else {
                            message.content().contains(&word.value.0)
                        }
                    })
                    .map(|word| word.value)
                    .collect::<Vec<_>>();

                if !matched_words.is_empty() {
                    let user = match self.user_repo.find_by_id(&user_id).await {
                        Ok(user) => user,
                        Err(e) => {
                            log::error!("Failed to find user: {}", e);
                            return;
                        }
                    };
                    let notification = WordNotification::new(
                        user.traq_uuid,
                        matched_words,
                        message.uuid().clone(),
                    );
                    if let Err(e) = self
                        .notification_service
                        .send_word_notification(notification)
                        .await
                    {
                        log::error!("Failed to send notification: {}", e);
                    }
                }
            })
            .await;

        Ok(())
    }
}
