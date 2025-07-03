use axum::Router;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use crate::{route::middleware::Auth, server::ServerState};

pub mod course;
pub mod department;
pub mod index;
pub mod login;
pub mod score;
pub mod student;

pub fn build_router() -> Router<ServerState> {
    Router::new()
        .nest("/index", index::router())
        .nest("/student", student::router())
        .nest("/score", score::router())
        .nest("/department", department::router())
        .nest("/course", course::router())
        .route_layer(AsyncRequireAuthorizationLayer::new(Auth))
        .nest("/login", login::router())
}