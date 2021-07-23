mod controllers;
mod services;
mod models;
mod repositories;
mod utils;
mod configs;
mod middlewares;
use std::sync::Arc;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use actix_session::{CookieSession, Session};
use elasticsearch::Elasticsearch;
// use oauth2::basic::BasicClient;

use argon2::Argon2;

#[get("/")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("api")
}

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
                .configure(|cfg| configure_user(elastic_client.clone(), cfg))
                .configure(|cfg| configure_artwork(elastic_client.clone(), cfg))
                .service(api)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn configure_auth(elastic_client: Arc<Elasticsearch>, cfg: &mut web::ServiceConfig){
    use crate::controllers::auth_controller;
    use crate::services::impls::auth_service_impl::AuthServiceImpl;
    use crate::repositories::impls::credential_repo_impl::CredentialRepoImpl;
    use crate::repositories::impls::user_repo_impl::UserRepoImpl;
    let service = AuthServiceImpl{
        credential_repo: CredentialRepoImpl{
            elastic_client: elastic_client.clone()
        },
        user_repo: UserRepoImpl {
            elastic_client: elastic_client.clone()
        },
        argon2: Argon2::default()
    };
    auth_controller::configure(web::Data::new(service), cfg);
}

fn configure_user(elastic_client: Arc<Elasticsearch>, cfg: &mut web::ServiceConfig){
    use crate::controllers::user_controller;
    use crate::repositories::impls::user_repo_impl::UserRepoImpl;
    let repo = UserRepoImpl {
        elastic_client: elastic_client.clone()
    };
    user_controller::configure(web::Data::new(repo), cfg);
}

fn configure_artwork(elastic_client: Arc<Elasticsearch>, cfg: &mut web::ServiceConfig){
    use crate::controllers::artwork_controller;
    use crate::repositories::impls::artwork_repo_impl::ArtworkRepoImpl;
    use crate::repositories::impls::user_repo_impl::UserRepoImpl;
    let artwork_repo = ArtworkRepoImpl {
        elastic_client: elastic_client.clone()
    };
    let user_repo = UserRepoImpl {
        elastic_client: elastic_client.clone()
    };
    artwork_controller::configure(web::Data::new(artwork_repo), web::Data::new(user_repo), cfg);
}
