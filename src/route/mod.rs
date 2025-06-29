use axum::Router;
use crate::error::AppError;
use crate::route::result::AppResult;
use crate::server::ServerState;

pub mod result;
pub mod request;
pub mod page;
mod extract;

pub fn build_router() -> Router<ServerState> {
    request::build_router()
        .fallback(not_found)
        .method_not_allowed_fallback(not_allowed)
}

async fn not_found() -> AppResult<()> {
    tracing::warn!("Not Found");
    AppResult::Err(AppError::NotFound)
}

async fn not_allowed() -> AppResult<()> {
    tracing::warn!("Method Not Allowed");
    AppResult::Err(AppError::MethodNotAllowed)
}