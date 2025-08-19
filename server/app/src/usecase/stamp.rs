use anyhow::Result;

use crate::{
    domain::{
        stamp::{NewStamp, StampRepository},
        traq_stamp::TraqStampId,
        user::UserId,
    },
    infra::repo::Repository,
};

pub struct StampService {
    repo: Repository,
}

impl StampService {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub async fn register_stamp(
        &self,
        user_id: UserId,
        traq_stamp_id: TraqStampId,
    ) -> Result<()> {
        let stamp = NewStamp::new(user_id, traq_stamp_id);
        self.repo.insert_stamp(stamp).await
    }
}
