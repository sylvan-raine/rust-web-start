use axum::{debug_handler, routing, Router};
use crate::route::not_found;
use crate::route::result::AppResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
     Router::new()
         .route("/", routing::get(query_index))
         .fallback(not_found)
}

#[debug_handler]
async fn query_index() -> AppResult<&'static str> {
    tracing::debug!("Query index");
    
    AppResult::Ok("Welcome! This is the index page of this site.")
}