use crate::models::{
    token::{Claims, GoogleClaims, JWKS},
    error::ServiceError,
};
use actix_web::{
    http::{
        StatusCode,
    },
};
use reqwest;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, decode_header, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use std::error::Error;
use crate::configs::var::{GOOGLE_CLIENT_ID, GOOGLE_JWK_URL, GOOGLE_AUTH_ISS, JWT_KEY};
use std::collections::HashSet;

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}
pub fn validate_google_id_token(token: &str) -> Result<GoogleClaims, ServiceError>{
    if (token.is_empty()){
        return Err(ServiceError::UnauthorizedError("id_token string empty".to_string()))
    }
    let header = match decode_header(&token){
        Ok(h) => h,
        Err(error) => return Err(ServiceError::UnauthorizedError("token header decode error: ".to_string() + &error.to_string()))
    };
    let jwks = match fetch_jwks(GOOGLE_JWK_URL) {
        Ok(j) => j,
        Err(error) => return Err(ServiceError::UnauthorizedError("jwk fetch -> ".to_string() + &error.to_string()))
    };
    let jwk = jwks.find(&header.kid.unwrap_or("".to_string())).expect("Specified key not found in set");
    let mut aud_set = HashSet::new();
    aud_set.insert(GOOGLE_CLIENT_ID.to_string());
    let validation = Validation { 
        iss: Some(GOOGLE_AUTH_ISS.to_string()), 
        aud: Some(aud_set),
        ..Validation::new(Algorithm::RS256)
    };
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is models::auth::GoogleClaims
    let token_data = match decode::<GoogleClaims>(&token, &DecodingKey::from_rsa_components(&jwk.n, &jwk.e), &validation){    
        Ok(t) => t,
        Err(error) => return match *error.kind() {
           ErrorKind::ExpiredSignature => Err(ServiceError::JWTExpireError),
           _ => Err(ServiceError::UnauthorizedError("toekn_util -> ".to_string() + &error.to_string())),
        }
    };
    Ok(token_data.claims)  
}

pub fn generate_jwt(email: &String) -> Result<String, ServiceError>{
    let claims = Claims { sub: email.to_owned(), exp: 10000000000 };
    match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_KEY)) {
        Ok(t) => Ok(t),
        Err(error) => Err(ServiceError::InternalServerError("Error generating jwt: ".to_string() + &error.to_string()))
    }

}

pub fn validate_jwt(token: &str) -> Result<Claims, ServiceError>{
    match decode::<Claims>(&token, &DecodingKey::from_secret(JWT_KEY), &Validation::default()) {
        Ok(token_data) => Ok(token_data.claims),
        Err(error) => return match *error.kind() {
           ErrorKind::ExpiredSignature => Err(ServiceError::JWTExpireError),
           _ => Err(ServiceError::UnauthorizedError("toekn_util -> ".to_string() + &error.to_string())),
        }
    }
}