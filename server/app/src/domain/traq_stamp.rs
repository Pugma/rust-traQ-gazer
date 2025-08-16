use anyhow::Result;
use uuid::Uuid;

pub struct TraqStampId(Uuid);

pub struct TraqStampName(String);

pub struct TraqStamp {
    uuid: TraqStampId,
    name: TraqStampName,
}

pub trait TraqStampService {
    async fn get_stamp_id_by_name(&self, name: &str) -> Result<TraqStampId>;
    async fn sync_stamps_from_traq(&self) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<TraqStamp>>;
}
