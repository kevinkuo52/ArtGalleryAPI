use crate::configs::var::{ARTWORK_INDEX};
use elasticsearch::{Elasticsearch, IndexParts, GetParts};
use async_trait::async_trait;
use serde_json::{json};
use std::sync::Arc;
use crate::repositories::artwork_repo_i::ArtworkRepo;
use crate::models::{
    artwork::{
        Artwork,
    },
    elastic_res::{DocES, IndexResES}
};
use crate::models::error::ServiceError;

pub struct ArtworkRepoImpl {
    pub elastic_client: Arc<Elasticsearch>
}

#[async_trait]
impl ArtworkRepo for ArtworkRepoImpl {


    async fn create_artwork(self: &Self, artwork: &Artwork) -> Result<String, ServiceError> {
        /*return _id of created artwork doc*/
        let result = self.elastic_client
            .index(IndexParts::Index(ARTWORK_INDEX))
            .body(json!(artwork))
            .send()
            .await;
        let response = match result {
            Ok(res) => res,
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        };
        if !response.status_code().is_success() {
            return Err(ServiceError::InternalServerError("ES http error saving artwork".to_string()))
        }
        match response.json::<IndexResES>().await {
            Ok(res) => Ok(res._id),
            Err(err) =>  Err(ServiceError::InternalServerError("ES error deserializing create artwork response: ".to_string() + &err.to_string()))
        }
    }

    async fn get_artwork(self: &Self, id: &String) -> Result<DocES<Artwork>, ServiceError> {
        let response = match self.elastic_client
            .get(GetParts::IndexId(ARTWORK_INDEX, id))
            .send()
            .await {
                Ok(res) => res,
                Err(error) => return Err(ServiceError::InternalServerError(
                    "ES get user error: ".to_string() + &error.to_string()
                )),
            };
        if !response.status_code().is_success(){
            return Err(ServiceError::InternalServerError("ES get artwork http err: ".to_string() + &response.status_code().as_str()))
        }
        match response.json::<DocES<Artwork>>().await {
            Ok(r) => Ok(r),
            Err(error) => return Err(ServiceError::InternalServerError("ES get artwork deserialization error: ".to_string() + &error.to_string())),
        }
    }
}