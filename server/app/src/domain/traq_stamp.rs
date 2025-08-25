use anyhow::Result;
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct TraqStampId(pub i64);

pub struct TraqStampName(pub String);

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct TraqStampUuid(pub Uuid);

pub struct TraqStamp {
    id: TraqStampId,
    name: TraqStampName,
    uuid: TraqStampUuid,
}

pub trait TraqStampService {
    async fn sync_with_traq(&self) -> Result<()>;
}

pub trait TraqStampRepository {
    async fn upsert_stamps(&self, stamps: Vec<TraqStamp>) -> Result<()>;
    async fn get_all_stamps(&self) -> Result<Vec<TraqStamp>>;
    async fn find_stamp_by_name(&self, name: &str) -> Result<Option<TraqStamp>>;
}
