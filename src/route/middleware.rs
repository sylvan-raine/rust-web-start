use std::{pin::Pin, sync::LazyLock};

use axum::{
    body::Body,
    extract::Request,
    http::{Response, header},
    response::IntoResponse,
};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

use crate::{
    error::AppError,
    route::{
        jwt::{DEFAULT_VALIDATION, Jwt},
        request::login::UserIdent,
    },
};

pub static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<Auth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::new(Auth));

#[derive(Clone)]
pub struct Auth;

impl AsyncAuthorizeRequest<Body> for Auth {
    type RequestBody = Body;

    type ResponseBody = Body;

    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        Box::pin(async move {
            let auth_header = request.headers().get(header::AUTHORIZATION);

            if auth_header.is_none() {
                Err(AppError::Unauthorized("你还未登录!".to_string()).into_response())
            } else {
                let auth_header = auth_header.unwrap().to_str().map_err(|e| {
                    AppError::BadRequest(format!("找到一个无法被现有编码支持的字符, 详细信息: {e}"))
                })?;

                let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
                    AppError::BadRequest("Authorization 字段应该以 \"Bearer \" 开头".to_string())
                })?;

                let usr_ident =
                    Jwt::<UserIdent>::decode_with(token, &DEFAULT_VALIDATION).map_err(|e| {
                        AppError::Unauthorized(format!("JWT 校验未通过, 详细信息: {e}"))
                    })?;
                request.extensions_mut().insert(usr_ident);

                Ok(request)
            }
        })
    }
}
