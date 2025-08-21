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
    async fn get_stamp_id_by_name(&self, name: &str) -> Result<TraqStampId>;
    async fn sync_stamps_from_traq(&self) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<TraqStamp>>;
}
