use anyhow::{Ok, Result};
mod handler;
mod repo;
mod traq;

use log::info;

use handler::Handler;
use repo::Repository;
use tokio::{net::TcpListener, spawn};

#[tokio::main]
async fn main() -> Result<()> {
    // enable env_logger
    env_logger::init();

    // setup DB connection
    let repo = Repository::setup().await.expect("Failed to access the DB!");
    info!("Made connections to the DB correctly!");

    // setup API server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = openapi::server::new(Handler { repo });
    spawn(async move {
        println!("Opening the endpoints ...");
        axum::serve(listener, app)
            .await
            .expect("Failed to open the endpoints!");
    });

    // setup message poller
    traq::start_polling().await?;

    Ok(())
}
