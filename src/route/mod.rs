use axum::Router;
use crate::route::result::AppResult;
use crate::server::ServerState;

pub mod middleware;
pub mod jwt;
pub mod result;
pub mod request;
pub mod page;
mod extract;

pub fn build_router() -> Router<ServerState> {
    request::build_router()
}