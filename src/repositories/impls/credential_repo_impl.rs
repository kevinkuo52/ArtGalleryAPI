
use crate::configs::var::{CREDENTIAL_INDEX};
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};
use async_trait::async_trait;
use serde_json::{json};
use std::sync::Arc;
use crate::repositories::credential_repo_i::CredentialRepo;
use crate::models::{
    auth::{
        Credential,
    },
    elastic_res::{SearchResES, DocES, IndexResES}
};
use crate::models::error::ServiceError;

pub struct CredentialRepoImpl {
    pub elastic_client: Arc<Elasticsearch>
}

#[async_trait]
impl CredentialRepo for CredentialRepoImpl {


    async fn create_credential(self: &Self, credential: &Credential) -> Result<String, ServiceError> {
        /* create credentail
        return _id of the created credential doc
        */
        let result = self.elastic_client
            .index(IndexParts::Index(CREDENTIAL_INDEX))
            .body(json!(credential))
            .send()
            .await;
        let response = match result {
            Ok(res) => res,
            Err(error) => return Err(ServiceError::InternalServerError(error.to_string())),
        };
        if !response.status_code().is_success() {
            return Err(ServiceError::InternalServerError("ES http error saving credentials".to_string()))
        }
        match response.json::<IndexResES>().await {
            Ok(res) => Ok(res._id),
            // Ok(res) => Ok(res["_id"].as_str().unwrap().to_string()),
            Err(err) =>  Err(ServiceError::InternalServerError("ES error deserializing create credential response: ".to_string() + &err.to_string()))
        }
    }

    async fn get_credential(self: &Self, email: &String) -> Result<DocES<Credential>, ServiceError> {
         let result = self.elastic_client
            .search(SearchParts::Index(&[CREDENTIAL_INDEX]))
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