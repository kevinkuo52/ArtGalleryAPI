use crate::repositories::user_repo_i::UserRepo;
use crate::utils::{constants, token_util};
use crate::models::{
    user::{
        User,
    },
    token::{
        Claims,
    },
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

pub fn configure<T: 'static + UserRepo>(user_repo: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(user_repo);
    // cfg.route("/user", web::get().to(register::<T>));
    cfg.route("/user", web::get().to(get_user::<T>));
}

// async fn register<T: UserRepo>(user_repo: web::Data<T>, body: Json<RegistrationReq>) -> Result<HttpResponse, ServiceError>  {
    
//     match service.register(&body, constants::NATIVE_AUTH_TYPE).await {
//         Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("User created", constants::EMPTY))),
//         Err(err) => Err(err),
//     }
// }

async fn get_user<T: UserRepo>(user_repo: web::Data<T>, claims: Claims) -> Result<HttpResponse, ServiceError> {
    // session.set("login", true).unwrap();
    // HttpResponse::Ok().json(ResponseBody::new("res", constants::EMPTY))
    match user_repo.get_user(&claims.sub).await {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new("", user._source))),
        Err(err) => Err(err),
    }
}
