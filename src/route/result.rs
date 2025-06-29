use crate::error::AppError;
use axum::response::IntoResponse;
use serde::Serialize;

/// 解决返回值为 [AppResult] 类型的函数中不能使用 `?` 运算符传递错误的不便
#[macro_export]
macro_rules! throw_err {
    ($err_or_success : expr) => {
        match $err_or_success {
            Ok(val) => val,
            Err(err) => return crate::route::AppResult::Err(crate::route::AppError::from(err))
        }
    };
}

#[derive(Serialize)]
pub struct Page<T: Serialize> {
    pub page_index: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub items: Vec<T>
}

#[derive(Serialize)]
pub enum AppResult<T: Serialize> {
    Err(AppError),
    Ok(T)
}

impl<T: Serialize> IntoResponse for AppResult<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppResult::Err(e) => e.into_response(),
            AppResult::Ok(val) => axum::Json(val).into_response()
        }
    }
}

impl<T: Serialize> From<sea_orm::DbErr> for AppResult<T> {
    fn from(value: sea_orm::DbErr) -> Self {
        AppResult::Err(AppError::from(value))
    }
}

impl<T: Serialize> From<anyhow::Error> for AppResult<T> {
    fn from(value: anyhow::Error) -> Self {
        AppResult::Err(AppError::from(value))
    }
}