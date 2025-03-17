use crate::repo::Repository;

mod error;
mod similar;
mod stamps;
mod trend;
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
