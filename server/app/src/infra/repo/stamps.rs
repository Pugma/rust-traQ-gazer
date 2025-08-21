use anyhow::Result;

use crate::{
    domain::{
        stamp::{NewStamp, Stamp, StampId, StampRepository},
        user::UserId,
    },
    infra::repo::Repository,
};

impl StampRepository for Repository {
    async fn register(&self, stamp: NewStamp) -> Result<()> {
        unimplemented!()
    }

    async fn find_all(&self) -> Result<Vec<Stamp>> {
        unimplemented!()
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Stamp>> {
        unimplemented!()
    }

    async fn delete(&self, stamp_id: &StampId) -> Result<()> {
        unimplemented!()
    }
}
