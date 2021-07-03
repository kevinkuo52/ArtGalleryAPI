use crate::utils::token_util;
use crate::utils::constants;
use crate::models::token::Claims;
use crate::models::error::ServiceError;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures_util::future::{ok, err, Ready};

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Claims, Self::Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                match  token_util::validate_jwt(token) {
                    Ok(claims) => ok(claims),
                    Err(error) => err(error.into())
                }
            }
            None => err(ServiceError::UnauthorizedError("Authorization Bearer header not found".to_string()).into()),
        }
    }
}