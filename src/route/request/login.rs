use crate::entity::prelude::Users;
use crate::entity::users::Model;
use crate::error::AppError;
use crate::route::extract::ValidJson;
use crate::route::jwt::Jwt;
use crate::route::middleware::AUTH_LAYER;
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;
use axum::extract::State;
use axum::{Extension, Router, debug_handler, routing};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/user-info", routing::get(info))
        .route_layer(&*AUTH_LAYER)
        .route("/", routing::post(login))
}

/// 登录参数
#[derive(Deserialize, Validate)]
struct Params {
    #[validate(length(min = 1, max = 32, message = "id 长度应该小于 32 而大于 1"))]
    id: String,
    #[validate(length(min = 1, max = 128, message = "password 长度应该小于 128 而大于 1"))]
    password: String,
}

/// 登陆完成后返回给浏览器的信息, 这将被存储在 jwt 中
#[derive(Serialize, Deserialize, Clone)]
pub struct UserIdent {
    pub id: String,
    pub name: String,
}

#[debug_handler]
#[tracing::instrument(name = "[登录]", skip_all, fields(account = %param.id))]
async fn login(
    State(state): State<ServerState>,
    ValidJson(param): ValidJson<Params>,
) -> AppResult<String> {
    tracing::info!("有用户试图登录! 登录账号: {}", param.id);
    let users = Users::find_by_id(param.id).one(state.db()).await;

    match throw_err!(users) {
        Some(usr) => {
            if usr.password == param.password {
                tracing::info!("登录成功!");
                let usr_ident = UserIdent {
                    id: usr.id,
                    name: usr.name,
                };
                let token = Jwt::generate(usr_ident);
                return AppResult::Ok(token);
            } else {
                tracing::warn!("此用户的账号与密码不匹配!")
            }
        }
        _ => {
            tracing::info!("此用户账号不存在!")
        }
    }
    AppResult::Err(AppError::Unauthorized("账号或者密码不正确!".to_string()))
}

#[debug_handler]
async fn info(
    State(state): State<ServerState>,
    Extension(usr): Extension<UserIdent>,
) -> AppResult<Model> {
    let entity = Users::find_by_id(usr.id)
        .one(state.db())
        .await
        .unwrap()
        .unwrap();

    AppResult::Ok(entity)
}
