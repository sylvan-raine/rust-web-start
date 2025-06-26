use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use crate::server::ServerState;

pub mod student;
pub mod index;
mod response;
mod sc;
mod department;
mod course;

pub fn build_router() -> Router<ServerState> {
    Router::new()
        .merge(index::router())
        .merge(student::router())
        .merge(sc::router())
        .merge(department::router())
        .merge(course::router())
        .fallback(not_found)
}

async fn not_found() -> impl IntoResponse {
    tracing::warn!("Not Found");
    (StatusCode::NOT_FOUND, "Resource not found")
}