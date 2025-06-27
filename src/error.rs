use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum AppError {
    #[error("Server lost it unintentionally. 😢")]
    NotFound,                       // 404 Not Found
    
    #[error("Why you do this to me! 🥲")]
    MethodNotAllowed,               // 405 Method Not Allowed
    
    #[error("Sorry, what's your query? 🤔 {0}")]
    BadQuery(String),               // 400 Bad Request
    
    #[error("Sorry, what's your JSON? 🤔 {0}")]
    BadJson(String),                // 400 Bad Request
    
    #[error("Sorry, what's your path? 🤔 {0}")]
    BadPath(String),                // 400 Bad Request
    
    #[error("Sorry, but check the params. 😢 {0}")]
    UnprocessableEntity(String),    // 422 Unprocessable Entity
    
    #[error("It's hard to tell you I broke down... 😶")]
    Internal(String),               // 服务器内部错误
    
    #[error("No... The database can't handle this. 😍")]
    Database(String),               // 数据库错误
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ResponseStruct {
            status_code: u16,
            error: AppError,
            to_client: String,
        }
        
        (self.status_code(), axum::Json(
            ResponseStruct {
                status_code: self.status_code().as_u16(),
                to_client: self.to_string(),
                error: self
            }
        )).into_response()
    }
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        use AppError::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
            MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            BadQuery(_) | BadJson(_) | BadPath(_) => StatusCode::BAD_REQUEST,
            UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Database(_) | Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<axum::extract::rejection::QueryRejection> for AppError {
    fn from(e: axum::extract::rejection::QueryRejection) -> Self {
        Self::BadQuery(e.to_string())
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