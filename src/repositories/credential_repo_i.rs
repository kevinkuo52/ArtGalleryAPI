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
    async fn save_credential(&self, credential: &Credential) -> Result<(), ServiceError>;
    async fn get_credential(&self, email: &String) ->Result<DocES<Credential>, ServiceError>;
}
