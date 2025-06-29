use axum::{debug_handler, routing, Router};
use axum::extract::State;
use sea_orm::EntityTrait;
use crate::entity::course::Model;
use crate::entity::prelude::Course;
use crate::route::not_found;
use crate::route::result::AppResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .fallback(not_found)
}

/// 路由到 course 模块下的默认界面
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("Welcome! This is the index page of course.")
}


/// 处理路由到 course 模块下的查询请求
#[debug_handler]
async fn query(State(state): State<ServerState>) -> AppResult<Vec<Model>> {
    tracing::debug!("Query course");
    let course = Course::find()
        .all(state.db())
        .await
        .unwrap();

    AppResult::Ok(course)
}