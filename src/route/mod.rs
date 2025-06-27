use axum::Router;
use crate::error::AppError;
use crate::route::result::QueryResult;
use crate::server::ServerState;

pub mod student;
pub mod index;
mod result;
mod score;
mod department;
mod course;

pub fn build_router() -> Router<ServerState> {
    Router::new()
        .merge(index::router())
        .merge(student::router())
        .merge(score::router())
        .merge(department::router())
        .merge(course::router())
        .fallback(not_found)
        .method_not_allowed_fallback(not_allowed)
}

async fn not_found() -> QueryResult<()> {
    tracing::warn!("Not Found");
    QueryResult::Err(AppError::NotFound)
}

async fn not_allowed() -> QueryResult<()> {
    tracing::warn!("Method Not Allowed");
    QueryResult::Err(AppError::MethodNotAllowed)
}