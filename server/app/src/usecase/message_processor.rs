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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        notification::StampNotification,
        traq_message::{TraqMessage, TraqMessageUuid},
        user::{NewUser, User, UserId},
        word::{NewWord, Word, WordId, WordUuid, WordValue},
    };
    use anyhow::anyhow;
    use std::{future::Future, sync::Mutex};
    use uuid::Uuid;

    struct FakeWord {
        target_user_id: UserId,
        value: &'static str,
        is_regex: bool,
        excluded_message_user_ids: Vec<UserId>,
    }

    struct FakeWordRepository {
        words: Vec<FakeWord>,
    }

    impl WordRepository for FakeWordRepository {
        async fn insert_word(&self, _word: NewWord) -> Result<()> {
            panic!("insert_word should not be called")
        }

        async fn get_all_words(&self) -> Result<Vec<Word>> {
            Ok(self
                .words
                .iter()
                .enumerate()
                .map(|(index, word)| Word {
                    id: WordId(index as i64 + 1),
                    uuid: WordUuid(Uuid::from_u128(index as u128 + 1)),
                    user_id: word.target_user_id,
                    value: WordValue(word.value.to_string()),
                    is_regex: word.is_regex,
                    excluded_message_user_ids: word.excluded_message_user_ids.clone(),
                })
                .collect())
        }

        async fn find_words_by_user_id(&self, _user_id: &UserId) -> Result<Vec<Word>> {
            panic!("find_words_by_user_id should not be called")
        }

        async fn delete_word(&self, _word_id: &WordId) -> Result<()> {
            panic!("delete_word should not be called")
        }
    }

    struct FakeUserRepository {
        target_user_id: UserId,
        target_user_uuid: Uuid,
    }

    impl UserRepository for FakeUserRepository {
        async fn upsert_users(&self, _users: Vec<NewUser>) -> Result<()> {
            panic!("upsert_users should not be called")
        }

        async fn find_by_id(&self, user_id: &UserId) -> Result<User> {
            if *user_id != self.target_user_id {
                return Err(anyhow!("unexpected user id"));
            }

            Ok(User {
                id: self.target_user_id,
                display_name: "Target User".to_string(),
                traq_id: "target".to_string(),
                traq_uuid: self.target_user_uuid,
                is_bot: false,
                is_expired: false,
            })
        }

        async fn find_by_traq_id(&self, _traq_id: &str) -> Result<User> {
            panic!("find_by_traq_id should not be called")
        }

        async fn find_by_traq_uuid(&self, _traq_uuid: Uuid) -> Result<User> {
            panic!("find_by_traq_uuid should not be called")
        }
    }

    #[derive(Debug, PartialEq)]
    struct RecordedWordNotification {
        target_user_uuid: Uuid,
        matched_word_values: Vec<String>,
        message_uuid: Uuid,
    }

    struct FakeNotificationService {
        notifications: Mutex<Vec<RecordedWordNotification>>,
    }

    impl FakeNotificationService {
        fn new() -> Self {
            Self {
                notifications: Mutex::new(Vec::new()),
            }
        }
    }

    impl NotificationService for FakeNotificationService {
        async fn send_word_notification(&self, notification: WordNotification) -> Result<()> {
            self.notifications
                .lock()
                .expect("notifications lock should not be poisoned")
                .push(RecordedWordNotification {
                    target_user_uuid: notification.target_user_uuid,
                    matched_word_values: notification
                        .matched_word_values
                        .into_iter()
                        .map(|word| word.0)
                        .collect(),
                    message_uuid: notification.message_uuid.0,
                });
            Ok(())
        }

        async fn send_stamp_notification(&self, _notification: StampNotification) -> Result<()> {
            panic!("send_stamp_notification should not be called")
        }
    }

    fn run_async<T>(future: impl Future<Output = T>) -> T {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime should build")
            .block_on(future)
    }

    fn message_from(author_user_id: UserId, content: &str, uuid: Uuid) -> TraqMessage {
        TraqMessage::new(
            TraqMessageUuid::new(uuid),
            author_user_id,
            content.to_string(),
            Vec::new(),
        )
    }

    #[test]
    fn sends_notification_when_word_matches() {
        run_async(async {
            let target_user_id = UserId(1);
            let target_user_uuid = Uuid::from_u128(100);
            let message_uuid = Uuid::from_u128(200);
            let notification_service = Arc::new(FakeNotificationService::new());
            let processor = MessageProcessor::new(
                Arc::new(FakeWordRepository {
                    words: vec![FakeWord {
                        target_user_id,
                        value: "traP",
                        is_regex: false,
                        excluded_message_user_ids: Vec::new(),
                    }],
                }),
                notification_service.clone(),
                Arc::new(FakeUserRepository {
                    target_user_id,
                    target_user_uuid,
                }),
            );

            processor
                .process_message(&message_from(UserId(2), "hello traP", message_uuid))
                .await
                .expect("message processing should succeed");

            let notifications = notification_service
                .notifications
                .lock()
                .expect("notifications lock should not be poisoned");
            assert_eq!(
                *notifications,
                vec![RecordedWordNotification {
                    target_user_uuid,
                    matched_word_values: vec!["traP".to_string()],
                    message_uuid,
                }]
            );
        });
    }

    #[test]
    fn does_not_notify_when_message_author_is_excluded() {
        run_async(async {
            let target_user_id = UserId(1);
            let excluded_user_id = UserId(2);
            let target_user_uuid = Uuid::from_u128(100);
            let notification_service = Arc::new(FakeNotificationService::new());
            let processor = MessageProcessor::new(
                Arc::new(FakeWordRepository {
                    words: vec![FakeWord {
                        target_user_id,
                        value: "traP",
                        is_regex: false,
                        excluded_message_user_ids: vec![excluded_user_id],
                    }],
                }),
                notification_service.clone(),
                Arc::new(FakeUserRepository {
                    target_user_id,
                    target_user_uuid,
                }),
            );

            processor
                .process_message(&message_from(
                    excluded_user_id,
                    "hello traP",
                    Uuid::from_u128(200),
                ))
                .await
                .expect("message processing should succeed");

            let notifications = notification_service
                .notifications
                .lock()
                .expect("notifications lock should not be poisoned");
            assert!(notifications.is_empty());
        });
    }
}
