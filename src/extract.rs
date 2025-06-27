use crate::error::AppError;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum_valid::HasValidate;
use axum::http::request::Parts;

macro_rules! impl_from_request {
    ($name: ident, $wrapper: ident, FromRequestParts) => {
        impl<S, T> FromRequestParts<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequestParts<S, Rejection = AppError>,
        {
            type Rejection = AppError;

            async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
                Ok($name(Valid::from_request_parts(parts, state).await?.0.0))
            }
        }
    };
    ($name: ident, $wrapper: ident, FromRequest) => {
        impl<S, T> FromRequest<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$wrapper<T>>: FromRequest<S, Rejection = AppError>,
        {
            type Rejection = AppError;

            async fn from_request(parts: Request, state: &S) -> Result<Self, Self::Rejection> {
                Ok($name(Valid::from_request(parts, state).await?.0.0))
            }
        }
    };
}


pub struct ValidQuery<T>(pub T);
pub struct ValidPath<T>(pub T);
pub struct ValidJson<T>(pub T);
impl_from_request!(ValidQuery, Query, FromRequestParts);
impl_from_request!(ValidPath, Path, FromRequestParts);
impl_from_request!(ValidJson, Json, FromRequest);

#[derive(FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(AppError))]
pub struct Valid<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(AppError))]
pub struct Query<T>(pub T);

impl<T> HasValidate for Query<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::extract::Json), rejection(AppError))]
pub struct Json<T>(pub T);

impl<T> HasValidate for Json<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppError))]
pub struct Path<T>(pub T);

impl<T> HasValidate for Path<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}