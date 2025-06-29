use axum::{debug_handler, routing, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;
use serde::Deserialize;
use validator::Validate;
use crate::entity::prelude::Users;
use crate::route::extract::ValidQuery;
use crate::route::result::AppResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
}

/// 路由到 login 模块下的默认界面
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("Welcome! This is the index page of login.")
}


/// 处理路由到 login 模块下的查询请求
#[derive(Deserialize, Validate)]
struct Params {
    #[validate(length(min = 1, max = 32, message = "id should be less than 32 and more than 1 characters"))]
    id: String,
    #[validate(length(min = 1, max = 128, message = "password should be more than 1 and less than 128 characters"))]
    password: String,
}

#[debug_handler]
async fn query(State(state): State<ServerState>, ValidQuery(param): ValidQuery<Params>) -> impl IntoResponse {
    tracing::debug!("user: {} trying to login with passwd: {}", param.id, param.password);
    let course = Users::find()
        .all(state.db())
        .await
        .unwrap();

    AppResult::Ok(course)
}