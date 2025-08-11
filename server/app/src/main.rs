use anyhow::{Ok, Result};
use infra::{handler::Handler, repo::Repository, traq};
use log::info;
use tokio::net::TcpListener;

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

    // setup API server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = openapi::server::new(Handler { repo: repo.clone() });

    let endpoint_handler = tokio::spawn(async move {
        info!("Opening the endpoints ...");
        axum::serve(listener, app)
            .await
            .expect("Failed to open the endpoints!");
    });

    // setup message poller
    traq::start_polling(repo).await?;

    endpoint_handler.await?;

    Ok(())
}
