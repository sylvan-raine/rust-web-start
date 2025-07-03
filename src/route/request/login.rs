use axum::{debug_handler, routing, Router};
use axum::extract::State;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::entity::prelude::Users;
use crate::error::AppError;
use crate::route::extract::ValidQuery;
use crate::route::jwt::Jwt;
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(query))
}

/// 登录参数
#[derive(Deserialize, Validate)]
struct Params {
    #[validate(length(min = 1, max = 32, message = "id should be less than 32 and more than 1 characters"))]
    id: String,
    #[validate(length(min = 1, max = 128, message = "password should be more than 1 and less than 128 characters"))]
    password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserIdent {
    pub id: String,
    pub name: String,
}

#[debug_handler]
async fn query(State(state): State<ServerState>, ValidQuery(param): ValidQuery<Params>) -> AppResult<String> {
    tracing::info!("user: {} trying to login with passwd: {}", param.id, param.password);
    let users = Users::find_by_id(param.id)
        .one(state.db())
        .await;

    match throw_err!(users) {
        Some(usr) => {
            if usr.password == param.password {
                tracing::info!("user: {} login success!", usr.id);
                let payback = UserIdent {
                    id: usr.id,
                    name: usr.name,
                };
                let token = Jwt::generate(payback);
                return AppResult::Ok(throw_err!(token));
            } else {
                tracing::warn!("Someone trying to login with incorrect passwd!")
            }
        },
        _ => {
            tracing::info!("Someone trying to login without an account!")
        }
    }
    AppResult::Err(AppError::Unauthorized("Password or account incorrect!".to_string()))
}