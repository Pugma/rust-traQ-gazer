use std::sync::Arc;

use crate::{
    infra::traq::{message_collector::TraqMessageCollector, user_fetcher::TraqUserFetcher},
    usecase::{BackgroundTasks, UseCase},
};
use anyhow::{Ok, Result};
use infra::{handler::Handler, repo::Repository};
use log::info;
use tokio::{net::TcpListener, spawn};

mod config;
mod domain;
mod infra;
mod usecase;

#[tokio::main(worker_threads = 100)]
async fn main() -> Result<()> {
    // enable env_logger
    env_logger::init();

    // setup DB connection
    let repo = Repository::setup().await.expect("Failed to access the DB!");
    info!("Made connections to the DB correctly!");

    let usecase = UseCase::new(repo.clone());

    // setup API server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = openapi::server::new(Handler { repo: repo.clone() });

    let endpoint_handler = tokio::spawn(async move {
        info!("Opening the endpoints ...");
        axum::serve(listener, app)
            .await
            .expect("Failed to open the endpoints!");
    });

    spawn(async move {
        info!("Starting background tasks ...");
        BackgroundTasks::new(
            repo.clone(),
            TraqMessageCollector::new(Arc::new(repo)),
            TraqUserFetcher::new(),
        )
        .start()
        .await;
    });

    endpoint_handler.await?;

    Ok(())
}
