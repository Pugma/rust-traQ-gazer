use crate::infra::repo::Repository;

pub struct UserSynchronizerService {
    repo: Repository,
}

impl UserSynchronizerService {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub async fn sync_with_traq(&self) -> Result<(), String> {
        unimplemented!()
    }
}
