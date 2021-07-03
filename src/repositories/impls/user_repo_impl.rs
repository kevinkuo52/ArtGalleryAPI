
use elasticsearch::{Elasticsearch, IndexParts};

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::repositories::user_repo_i::UserRepo;
use crate::models::user::{
    User
};
use crate::models::error::ServiceError;
use crate::utils::constants;
use actix_web::{
    http::{
        StatusCode,
    },
};
// pub async fn configure() -> redis::Client {
//     let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env var needs to be set");
//     redis::Client::open(redis_url).expect("Unable to connect to Redis")
// }
pub struct UserRepoImpl {
    pub elastic_client: Arc<Elasticsearch>
}

#[async_trait]
impl UserRepo for UserRepoImpl {


    async fn save_user(self: &Self, user: &User) -> Result<bool, ServiceError> {
        let result = self.elastic_client
            .index(IndexParts::IndexId("user", &user.email))
            .body(json!(user))
            .send()
            .await;
        match result {
            Ok(response) => Ok(response.status_code().is_success()),
            Err(error) => Err(ServiceError::InternalServerError(error.to_string())),
        }
    }

}
