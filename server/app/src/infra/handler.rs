use crate::infra::repo::Repository;

mod error;
mod me;
mod me_stamps;
mod me_words;
mod stamps;
mod words;

#[derive(Clone)]
pub struct Handler {
    pub repo: Repository,
}

impl AsRef<Handler> for Handler {
    fn as_ref(&self) -> &Handler {
        self
    }
}
