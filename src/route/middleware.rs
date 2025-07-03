use std::pin::Pin;

use axum::{body::Body, 
    extract::Request, 
    http::{header, Response}, response::IntoResponse
};
use tower_http::auth::AsyncAuthorizeRequest;

use crate::{error::AppError, 
    route::{
        jwt::{Jwt, DEFAULT_VALIDATION}, 
        request::login::UserIdent
    }
};

#[derive(Clone)]
pub struct Auth;

impl AsyncAuthorizeRequest<Body> for Auth {
    type RequestBody = Body;

    type ResponseBody = Body;

    type Future = 
    Pin<
        Box<
            dyn Future<
                Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>
            > + Send
        >
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        Box::pin(async move {
            let auth_header = request.headers()
                .get(header::AUTHORIZATION);

            if let None = auth_header {
                Err(AppError::Unauthorized("You have not logged in!".to_string()).into_response())
            } else {
                let auth_header = auth_header.unwrap().to_str()
                    .map_err(|e| AppError::BadRequest(format!("found bytes opaque: {e}")))?;

                let token = auth_header.strip_prefix("Bearer ")
                    .ok_or_else(|| AppError::BadRequest("auth should begin with \"Bearer: \"".to_string()))?;

                let usr_ident = Jwt::<UserIdent>::decode_with(token, &DEFAULT_VALIDATION)
                    .map_err(|e| AppError::Unauthorized(format!("Auth not passed: {e}")))?;
                request.extensions_mut().insert(usr_ident);
                
                Ok(request)
            }
        })
    }
}