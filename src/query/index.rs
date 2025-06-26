use axum::{debug_handler, routing, Router};
use axum::response::IntoResponse;
use crate::query::response::QueryResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
     Router::new()
         .route("/", routing::get(query_index))
}

#[debug_handler]
async fn query_index() -> impl IntoResponse {
    tracing::debug!("Query index");
    
    QueryResult::Ok("index")
}