
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::repositories::credential_repo_i::CredentialRepo;
use crate::models::{
    auth::{
        Credential,
        CredentialES
    },
    elastic_res::{SearchResES, DocES}
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
            .index(IndexParts::Index("credential"))
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

    async fn get_credential(self: &Self, email: &String) -> Result<DocES<Credential>, ServiceError> {
         let result = self.elastic_client
            .search(SearchParts::Index(&["credential"]))
            .body(json!({
                "query": {
                    "term": {
                        "email": {
                            "value": email,
                        }
                    }
                }
            }))
            .send()
            .await;
        let response = match result {
            Ok(res) => res,
            Err(error) => return Err(ServiceError::InternalServerError(
                "ES get credential error: ".to_string() + &error.to_string()
            )),
        };
        // print!("repo before match");
        // match response.text().await {
        //     Ok(r) => print!("{}",r),
        //     Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        // };
        match response.json::<SearchResES<Credential>>().await {
            Ok(r) => Ok(r.hits.hits[0].clone()),
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        }
    }

}