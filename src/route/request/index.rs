use axum::{debug_handler, routing, Router};
use axum::response::IntoResponse;
use crate::route::not_found;
use crate::route::result::QueryResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
     Router::new()
         .route("/", routing::get(query_index))
         .fallback(not_found)
}

#[debug_handler]
async fn query_index() -> impl IntoResponse {
    tracing::debug!("Query index");
    
    QueryResult::Ok("Welcome! This is the index page of this site.")
}