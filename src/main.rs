use axum::{debug_handler, routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", routing::get(index));
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello world!"
}
