use crate::utils::token_util;
use crate::utils::constants;
use crate::models::token::GoogleClaims;
use crate::models::error::ServiceError;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures_util::future::{ok, err, Ready};
use actix_web::{
    http::{
        StatusCode,
    },
};

impl FromRequest for GoogleClaims {
    type Error = Error;
    type Future = Ready<Result<GoogleClaims, Self::Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                match  token_util::validate_google_id_token(token) {
                    Ok(claims) => ok(claims),
                    // Err(error) => err(error.response()),
                    Err(error) => err(error.into())
                    // Err(error) => err(ErrorUnauthorized("error: middlewares -> google_auth -> ".to_string() + &error.body.message)),
                }
            }
            // None => err(ServiceError::new(StatusCode::UNAUTHORIZED, "Authorization Bearer header not found", constants::EMPTY)),
            None => err(ServiceError::UnauthorizedError("Authorization Bearer header not found".to_string()).into()),
        }
    }
}