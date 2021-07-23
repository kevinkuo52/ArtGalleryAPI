use crate::repositories::artwork_repo_i::ArtworkRepo;
use crate::repositories::user_repo_i::UserRepo;
use crate::models::{
    artwork::{CreateArtworkReq, CreateArtworkRes},
    token::{Claims},
    response::ResponseBody,
    error::ServiceError,
};
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Result};

pub fn configure<A: 'static + ArtworkRepo, U: 'static + UserRepo>(artwork_repo: web::Data<A>, user_repo: web::Data<U>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(artwork_repo);
    cfg.route("/artwork", web::post().to(create_artwork::<A,U>));
    cfg.route("/artwork/{id}", web::get().to(get_artwork::<A>));
}

async fn create_artwork<A: ArtworkRepo, U: UserRepo>(artwork_repo: web::Data<A>, user_repo: web::Data<U>, claims: Claims, req: Json<CreateArtworkReq>) -> Result<HttpResponse, ServiceError>  {
    let artwork = req.to_model(&claims.sub);
    let artwork_id = artwork_repo.create_artwork(&artwork).await?;
    user_repo.add_artwork(&claims.sub, &artwork_id).await?;
    Ok(HttpResponse::Ok().json(ResponseBody::new(
        "Artwork created successfully", 
        CreateArtworkRes{artwork_id: artwork_id}
    )))
}

async fn get_artwork<A: ArtworkRepo>(artwork_repo: web::Data<A>, claims: Claims, id: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let artwork_es = artwork_repo.get_artwork(&id).await?;
    let artwork_res = artwork_es._source.to_artwork_res(&artwork_es._id);
    Ok(HttpResponse::Ok().json(ResponseBody::new("", artwork_res)))
}
