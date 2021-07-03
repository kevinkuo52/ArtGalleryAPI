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
    cfg.route("/register", web::post().to(register::<T>));
    cfg.route("/login", web::post().to(login::<T>));
    cfg.route("/logout", web::post().to(logout));
    cfg.route("/auth", web::get().to(auth));
}

async fn register<T: AuthService>(service: web::Data<T>, body: Json<RegistrationReq>) -> Result<HttpResponse, ServiceError>  {
    
    match service.register(&body, constants::NATIVE_AUTH_TYPE).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("User created", constants::EMPTY))),
        Err(err) => Err(err),
    }
}

async fn login<T: AuthService>(service: web::Data<T>, body: Json<LoginReq>) -> Result<HttpResponse, ServiceError> {
    // session.set("login", true).unwrap();
    let _ = match service.verify_password(&body).await {
        Ok(true) => true,//Ok(HttpResponse::Ok().json(ResponseBody::new("User logged in", constants::EMPTY))),
        Ok(false) => return Ok(HttpResponse::Ok().json(ResponseBody::new("Invalid username or password", constants::EMPTY))),
        Err(err) => return Err(err),
    };
    let token = token_util::generate_jwt(&body.email)?;

    Ok(HttpResponse::Ok().json(ResponseBody::new("Successful Login", LoginRes{access_token: token})))

}

fn logout(claims: GoogleClaims) -> HttpResponse {

    HttpResponse::Ok().json(ResponseBody::new("success", claims))
}

fn auth(claims: Claims) -> HttpResponse {
    HttpResponse::Ok().json(ResponseBody::new("success", claims.sub))
}

