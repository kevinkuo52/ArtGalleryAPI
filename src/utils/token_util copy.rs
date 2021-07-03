use crate::models::{
    token::{GoogleClaims, JWKS},
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
use crate::configs::var::{GOOGLE_CLIENT_ID, GOOGLE_JWK_URL, GOOGLE_AUTH_ISS};
use std::collections::HashSet;

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}
pub fn validate_google_id_token(token: &str) -> Result<GoogleClaims, ServiceError>{
    if (token.is_empty()){
        return Err(ServiceError::new(StatusCode::UNAUTHORIZED, "id_token string empty".to_string()))
    }
    let header = match decode_header(&token){
        Ok(h) => h,
        Err(error) => return Err(ServiceError::new(StatusCode::UNAUTHORIZED, "token header decode error: ".to_string() + &error.to_string()))
    };
    let jwks = match fetch_jwks(GOOGLE_JWK_URL) {
        Ok(j) => j,
        Err(error) => return Err(ServiceError::new(StatusCode::UNAUTHORIZED, error.to_string()))
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
        Err(err) => return Err(ServiceError::new(StatusCode::UNAUTHORIZED, "toekn_util -> ".to_string() + &err.to_string())),
        
    };
    Ok(token_data.claims)  
}