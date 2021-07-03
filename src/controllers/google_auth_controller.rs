use crate::services::auth_service_i::*;
use crate::utils::{constants, token_util};
use crate::models::{
    auth::{
        RegistrationReq,
        RegistrationRes,
        Credential,
        LoginReq,
        LoginRes,
    },
    token::{
        Claims,
        GoogleClaims
    },
    app_state::AppState,
    response::ResponseBody,
    error::ServiceError,
};
// use actix_session::{CookieSession, Session};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope,
};
use actix_web::http::header;
use actix_web::{web, Responder, HttpResponse, Result};
use actix_web::web::Json;

pub fn configure<T: 'static + AuthService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/google_register", web::post().to(register::<T>));
    cfg.route("/google_login", web::post().to(login::<T>));
    cfg.route("/google_logout", web::post().to(logout));

}

async fn register<T: AuthService>(service: web::Data<T>,claims: GoogleClaims) -> Result<HttpResponse, ServiceError>  {
    let body = RegistrationReq{email: claims.email: password: ""}
    match service.register(&body, constants::GOOGLE_AUTH_TYPE).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ResponseBody::new("User created", constants::EMPTY))),
        Ok(false) => Ok(HttpResponse::NotModified().json(ResponseBody::new("Failed to create user", constants::EMPTY))),
        Err(err) => Err(err),
    }
}

async fn login<T: AuthService>(service: web::Data<T>, claims: GoogleClaims) -> Result<HttpResponse, ServiceError> {
    //TODO: check if user is registered
    let token = token_util::generate_jwt(&body.email)?;
    Ok(HttpResponse::Ok().json(ResponseBody::new("Successful Login", LoginRes{access_token: token})))

}

fn logout(claims: GoogleClaims) -> HttpResponse {

    HttpResponse::Ok().json(ResponseBody::new("success", claims))
}