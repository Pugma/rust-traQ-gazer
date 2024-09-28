use anyhow::{Ok, Result};
mod handler;
mod repo;
mod traq;

use handler::Handler;
use repo::Repository;
use tokio::{net::TcpListener, time, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // setup DB connection
    let repo = Repository::setup().await.expect("Failed to access the DB!");
    println!("Made connections to the DB correctly!s");

    // setup API server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = openapi::server::new(Handler { repo });
    axum::serve(listener, app)
        .await
        .expect("Failed to open the endpoints!");
    println!("Opened the endpoints correctly!s");

    // setup message poller
    tokio::spawn(async {
        loop {
            // 3 分おきに実行
            let mut interval = time::interval(Duration::new(180, 0));
            interval.tick().await;

            let _ = traq::message::collect_message().await;
        }
    });

    Ok(())
}
