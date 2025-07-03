use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum AppError {
    #[error("服务器好像把它弄丢了. 😢 {0}")]
    NotFound(String),               // 404 Not Found
    
    #[error("不要这样对我! 🥲")]
    MethodNotAllowed,               // 405 Method Not Allowed
    
    #[error("你这请求是啥啊? 🤔 {0}")]
    BadRequest(String),             // 400 Bad Request
    
    #[error("你这 JSON 不对吧? 🤔 {0}")]
    BadJson(String),                // 400 Bad Request
    
    #[error("你的路径好像不对? 🤔 {0}")]
    BadPath(String),                // 400 Bad Request

    #[error("不是你谁啊, 先登录. 😢 {0}")]
    Unauthorized(String),           // 401 Unauthorized
    
    #[error("你请求参数取值好像不对, 服务器没法处理. 😢 {0}")]
    UnprocessableEntity(String),    // 422 Unprocessable Entity
    
    #[error("坏了, 服务器出问题了... 😶 {0}")]
    Internal(String),               // 500 服务器内部错误
    
    #[error("数据库应该出问题了. 😍")]
    Database(String),               // 500 数据库错误
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ResponseStruct {
            status_code: u16,
            status: String,
            error: AppError
        }
        
        (self.status_code(), axum::Json(
            ResponseStruct {
                status_code: self.status_code().as_u16(),
                status: self.status_code().to_string(),
                error: self
            }
        )).into_response()
    }
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        use AppError::*;
        match self {
            NotFound(_) => StatusCode::NOT_FOUND,
            MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            BadRequest(_) | BadJson(_) | BadPath(_) => StatusCode::BAD_REQUEST,
            Unauthorized(_) => StatusCode::UNAUTHORIZED,
            UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Database(_) | Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Into<AppError> for anyhow::Error {
    fn into(self) -> AppError {
        AppError::Internal(self.to_string())
    }
}

impl From<axum::extract::rejection::QueryRejection> for AppError {
    fn from(e: axum::extract::rejection::QueryRejection) -> Self {
        Self::BadRequest(e.to_string())
    }
}

impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(e: axum::extract::rejection::JsonRejection) -> Self {
        Self::BadJson(e.to_string())
    }
}

impl From<axum::extract::rejection::PathRejection> for AppError {
    fn from(e: axum::extract::rejection::PathRejection) -> Self {
        Self::BadPath(e.to_string())
    }
}

impl From<axum_valid::ValidRejection<AppError>> for AppError {
    fn from(e: axum_valid::ValidRejection<AppError>) -> Self {
        use AppError::*;
        match e {
            axum_valid::ValidRejection::Valid(v) => UnprocessableEntity(v.to_string()),
            axum_valid::ValidRejection::Inner(i) => i,
        }
    }
}

impl Into<AppError> for sea_orm::DbErr {
    fn into(self) -> AppError {
        AppError::Database(self.to_string())
    }
}

impl From<AppError> for axum::http::Response<Body> {
    fn from(value: AppError) -> Self {
        value.into_response()
    }
}