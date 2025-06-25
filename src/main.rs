mod logger;
mod app_config;
mod database;

use axum::{debug_handler, routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let _ = database::init().await?;
    let router = Router::new()
        .route("/", routing::get(index));
    
    let port = app_config::get_server().port();
    
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    tracing::info!("listening on {}.", listener.local_addr()?);
    axum::serve(listener, router).await?;
    
    Ok(())
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello world!"
}
