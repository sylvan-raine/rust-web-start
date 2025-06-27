use crate::error::AppError;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct Page<T: Serialize> {
    pub page_index: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub items: Vec<T>
}

#[derive(Serialize)]
pub enum QueryResult<T: Serialize> {
    Err(AppError),
    Ok(T)
}

impl<T: Serialize> IntoResponse for QueryResult<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            QueryResult::Err(e) => e.into_response(),
            QueryResult::Ok(val) => axum::Json(val).into_response()
        }
    }
}

impl<T: Serialize> From<sea_orm::DbErr> for QueryResult<T> {
    fn from(value: sea_orm::DbErr) -> Self {
        QueryResult::Err(AppError::from(value))
    }
}

impl<T: Serialize> From<anyhow::Error> for QueryResult<T> {
    fn from(value: anyhow::Error) -> Self {
        QueryResult::Err(AppError::from(value))
    }
}