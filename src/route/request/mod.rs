use axum::Router;
use crate::{route::middleware::AUTH_LAYER, server::ServerState};

pub mod course;
pub mod department;
pub mod index;
pub mod login;
pub mod score;
pub mod student;
pub mod resource;

pub fn build_router() -> Router<ServerState> {
    Router::new()
        .nest("/index", index::router())
        .nest("/student", student::router())
        .nest("/score", score::router())
        .nest("/department", department::router())
        .nest("/course", course::router())
        .route_layer(&*AUTH_LAYER)
        .nest("/login", login::router())
        .nest("/resource", resource::router())
}