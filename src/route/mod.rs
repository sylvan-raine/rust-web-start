use crate::route::result::AppResult;
use crate::server::ServerState;
use axum::Router;

mod extract;
pub mod jwt;
pub mod middleware;
pub mod page;
pub mod request;
pub mod result;

pub fn build_router() -> Router<ServerState> {
    request::build_router()
}
