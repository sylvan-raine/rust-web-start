use axum::{debug_handler, routing, Router};
use crate::route::not_found;
use crate::route::result::AppResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
     Router::new()
         .route("/", routing::get(query_index))
         .fallback(not_found)
}

#[debug_handler]
async fn query_index() -> AppResult<&'static str> {
    tracing::debug!("处理获取首页的请求");
    
    AppResult::Ok("欢迎! 这是本站点的首页.")
}