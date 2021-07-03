
use elasticsearch::{Elasticsearch, IndexParts, GetParts};

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::repositories::credential_repo_i::CredentialRepo;
use crate::models::auth::{
    Credential,
    CredentialES
};
use crate::models::error::ServiceError;
use actix_web::{
    http::{
        StatusCode,
    },
};

pub struct CredentialRepoImpl {
    pub elastic_client: Arc<Elasticsearch>
}

#[async_trait]
impl CredentialRepo for CredentialRepoImpl {


    async fn save_credential(self: &Self, credential: &Credential) -> Result<(), ServiceError> {
        let result = self.elastic_client
            .index(IndexParts::IndexId("credential", &credential.email))
            .body(json!(credential))
            .send()
            .await;
        let res_status = match result {
            Ok(response) => response.status_code().is_success(),
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        };
        match res_status {
            true => Ok(()),
        false => Err(ServiceError::InternalServerError("ES error saving credentials".to_string()))
        }
    }

    async fn get_credential(self: &Self, email: &String) -> Result<Credential, ServiceError> {
         let result = self.elastic_client
            .get(GetParts::IndexId("credential", email))
            .send()
            .await;
        let response = match result {
            Ok(res) => res,
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        };
        // print!("repo before match");
        // match response.text().await {
        //     Ok(r) => print!("{}",r),
        //     Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        // };
        match response.json::<CredentialES>().await {
            Ok(r) => Ok(r._source),
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        }
    }

}