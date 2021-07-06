use async_trait::async_trait;
use crate::models::{
    auth::{
        Credential,
    },
    elastic_res::{DocES}
};
use crate::models::error::ServiceError;
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepo  {
    async fn create_credential(&self, credential: &Credential) -> Result<String, ServiceError>;
    async fn get_credential(&self, email: &String) ->Result<DocES<Credential>, ServiceError>;
}
