use axum::{debug_handler, handler::HandlerWithoutStateExt, Router};
use tower_http::services::ServeDir;
use crate::{error::AppError, route::{middleware::AUTH_LAYER, result::AppResult}, server::ServerState};

pub mod course;
pub mod department;
pub mod login;
pub mod score;
pub mod student;

pub fn build_router() -> Router<ServerState> {
    let api = Router::new()
        .nest("/student", student::router())
        .nest("/score", score::router())
        .nest("/department", department::router())
        .nest("/course", course::router())
        .route_layer(&*AUTH_LAYER)
        .nest("/login", login::router());

    Router::new()
        .fallback_service(not_found.into_service())
        .method_not_allowed_fallback(not_allowed)
        .fallback_service(ServeDir::new("./static"))
        .nest("/api", api)
}

#[debug_handler]
async fn not_allowed() -> AppResult<()> {
    tracing::warn!("Method Not Allowed");
    AppResult::Err(AppError::MethodNotAllowed)
}

#[debug_handler]
async fn not_found() -> AppResult<()> {
    tracing::warn!("请求的资源不存在");
    AppResult::Err(AppError::NotFound(String::new()))
}