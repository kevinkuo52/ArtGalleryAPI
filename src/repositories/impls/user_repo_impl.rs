use crate::configs::var::{USER_INDEX};
use elasticsearch::{Elasticsearch, IndexParts, GetParts, UpdateParts};
use async_trait::async_trait;
use serde_json::{json};
use std::sync::Arc;
use crate::repositories::user_repo_i::UserRepo;
use crate::models::{
    user::{
        User,
    },
    elastic_res::{DocES}
};
use crate::models::error::ServiceError;

pub struct UserRepoImpl {
    pub elastic_client: Arc<Elasticsearch>
}

#[async_trait]
impl UserRepo for UserRepoImpl {


    async fn create_user(self: &Self, id: &String, username: &String) -> Result<(), ServiceError> {
        let user = User{
            username: username.to_string(),
            artworks: Vec::new(),
            description: "".to_string(),
            liked_artworks: Vec::new(),
            followers: Vec::new(),
            following: Vec::new(),
        };
        let result = self.elastic_client
            .index(IndexParts::IndexId(USER_INDEX, id))
            .body(json!(user))
            .send()
            .await;
        let res_status = match result {
            Ok(response) => response.status_code().is_success(),
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        };
        match res_status {
            true => Ok(()),
            false => Err(ServiceError::InternalServerError("ES create user http status err".to_string()))
        }
    }

    async fn get_user(self: &Self, id: &String) -> Result<DocES<User>, ServiceError> {
        let response = match self.elastic_client
            .get(GetParts::IndexId(USER_INDEX, id))
            .send()
            .await {
                Ok(res) => res,
                Err(error) => return Err(ServiceError::InternalServerError(
                    "ES get user error: ".to_string() + &error.to_string()
                )),
            };
        if !response.status_code().is_success(){
            return Err(ServiceError::InternalServerError("ES user http err: ".to_string() + &response.status_code().as_str()))
        }
        match response.json::<DocES<User>>().await {
            Ok(r) => Ok(r),
            Err(error) => return Err(ServiceError::InternalServerError("ES user deserialization error: ".to_string() + &error.to_string())),
        }
    }

    async fn add_artwork(self: &Self, user_id: &String, artwork_id: &String) -> Result<(), ServiceError> {
        let result = self.elastic_client
            .update(UpdateParts::IndexId(USER_INDEX, user_id))
            .body(json!({
                    "script": {
                        "source": "ctx._source.artworks.add(params.artwork)",
                        "lang": "painless",
                        "params": {
                            "artwork": artwork_id
                        }
                    }
                }))
            .send()
            .await;
        let res_status = match result {
            Ok(response) => response.status_code().is_success(),
            Err(error) => return Err(ServiceError::InternalServerError("ES user add artwork err: ".to_string() + &error.to_string())),
        };
        match res_status {
            true => Ok(()),
            false => Err(ServiceError::InternalServerError("ES user add artwork http status err".to_string()))
        }
    }
}