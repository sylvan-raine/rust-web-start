use axum::{debug_handler, routing, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;
use crate::entity::prelude::Course;
use crate::query::response::QueryResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/course", routing::get(query))
}

#[debug_handler]
async fn query(State(state): State<ServerState>) -> impl IntoResponse {
    tracing::debug!("Query course");
    let course = Course::find()
        .all(state.db())
        .await
        .unwrap();

    QueryResult::Ok(course)
}