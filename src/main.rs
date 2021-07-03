mod controllers;
mod services;
mod models;
mod repositories;
mod utils;
mod configs;
mod middlewares;
use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, dev::ServiceRequest};
// use actix_session::{CookieSession, Session};
use elasticsearch::Elasticsearch;
use models::error::ServiceError;
// use oauth2::basic::BasicClient;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use argon2::Argon2;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("root")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// async fn token_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, ServiceError> {
//     match utils::token::validate_google_id_token(credentials.token()) {
//         Ok(claims) => Ok(req),
//         Err(error) => Err(error),
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| {
    //     App::new().configure(|cfg| configure_auth(cfg))
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    //         // .route("/users", web::get().to(controllers::auth_controller::AuthController::register))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
    
    let elastic_client = Arc::new(Elasticsearch::default());
    HttpServer::new(move || {
            // let google_auth = HttpAuthentication::bearer(token_validator);
            App::new()
                // .data(AppState { oauth: configs::google_auth::configure() })
                // .wrap(CookieSession::signed(&[0; 32]).secure(false))
                // .wrap(google_auth)
                .configure(|cfg| configure_auth(elastic_client.clone(), cfg))
                .service(hello)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn configure_auth(elastic_client: Arc<Elasticsearch>, cfg: &mut web::ServiceConfig){
    use crate::controllers::auth_controller;
    use crate::services::impls::auth_service_impl::AuthServiceImpl;
    use crate::repositories::impls::credential_repo_impl::CredentialRepoImpl;
    let service = AuthServiceImpl{
        credential_repo: CredentialRepoImpl{
            elastic_client: elastic_client.clone()
        },
        argon2: Argon2::default()
    };
    auth_controller::configure(web::Data::new(service), cfg);
}

