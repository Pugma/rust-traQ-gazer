use anyhow::Result;
use sqlx::query;

use crate::domain::{
    stamp::{NewStamp, Stamp, StampId, StampRepository},
    user::UserId,
};

use super::Repository;

impl StampRepository for Repository {
    async fn insert_stamp(&self, stamp: &NewStamp) -> Result<()> {
        let result = query!(
            r#"
                INSERT INTO
                    `stamps` (`stamp_uuid`, `user_id`, `word`)
                VALUES
                    (?, ?, ?)
            "#,
            stamp.uuid(),
            stamp.user_id(),
            stamp.traq_stamp_id()
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    async fn get_all_stamps(&self) -> Result<Vec<Stamp>> {
        unimplemented!()
    }

    async fn find_stamps_by_user_id(&self, _user_id: &UserId) -> Result<Vec<Stamp>> {
        unimplemented!()
    }

    async fn delete_stamp(&self, _stamp_id: &StampId) -> Result<()> {
        unimplemented!()
    }
}
