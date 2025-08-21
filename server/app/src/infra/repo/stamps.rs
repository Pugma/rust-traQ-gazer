use crate::{
    domain::{
        stamp::{NewStamp, Stamp, StampId, StampRepository},
        user::UserId,
    },
    infra::repo::Repository,
};

impl StampRepository for Repository {
    async fn register(&self, stamp: NewStamp) -> Result<(), String> {
        unimplemented!()
    }

    async fn find_all(&self) -> Result<Vec<Stamp>, String> {
        unimplemented!()
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Stamp>, String> {
        unimplemented!()
    }

    async fn delete(&self, stamp_id: &StampId) -> Result<(), String> {
        unimplemented!()
    }
}
