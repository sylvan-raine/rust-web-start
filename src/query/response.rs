use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum QueryResult<T: Serialize> {
    Err(i32),
    Ok(T)
}

impl<T: Serialize> IntoResponse for QueryResult<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            QueryResult::Err(e) => format!("Server returned {e} as return value.").into_response(),
            QueryResult::Ok(val) => axum::Json(val).into_response()
        }
    }
}