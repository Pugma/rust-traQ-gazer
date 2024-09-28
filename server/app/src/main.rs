use anyhow::{Ok, Result};
mod handler;
mod repo;
mod traq;

use handler::Handler;
use repo::Repository;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // setup DB connection
    let repo = Repository::setup().await.expect("Failed to access the DB!");
    println!("Made connections to the DB correctly!");

    // setup API server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = openapi::server::new(Handler { repo });
    axum::serve(listener, app)
        .await
        .expect("Failed to open the endpoints!");
    println!("Opened the endpoints correctly!");

    // setup message poller
    traq::start_polling().await?;

    Ok(())
}
