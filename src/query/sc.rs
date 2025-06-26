use axum::{debug_handler, routing, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;
use crate::entity::prelude::Sc;
use crate::query::response::QueryResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/sc", routing::get(query))
}

#[debug_handler]
async fn query(State(state): State<ServerState>) -> impl IntoResponse {
    tracing::debug!("query sc table");
    let sc = Sc::find()
        .all(state.db())
        .await
        .unwrap();
    
    QueryResult::Ok(sc)
}