use axum::{debug_handler, extract::Path, http::StatusCode, response::{Html, IntoResponse, Response}, routing, Router};

use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/bin/{*path_to_resource}", routing::get(query_binary_resource))
        .route("/txt/{*path_to_resource}", routing::get(query_text_resource))

}

const PREFIX: &'static str = "./static/";

#[debug_handler]
async fn query_binary_resource(Path(mut path): Path<String>) -> Response {
    tracing::info!("正在读取文件并处理请求");
    path.insert_str(0, PREFIX);
    match tokio::fs::read(path).await {
        Ok(res) => (StatusCode::OK, res).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response()
    }
}

#[debug_handler]
async fn query_text_resource(Path(mut path): Path<String>) -> Response {
    path.insert_str(0, PREFIX);
    match tokio::fs::read_to_string(path).await {
        Ok(res) => (StatusCode::OK, Html(res)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response()
    }
}