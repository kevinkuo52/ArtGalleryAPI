use async_trait::async_trait;
use crate::models::user::{
    User,
};
use crate::models::error::ServiceError;
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepo  {
    async fn save_user(&self, user: &User) -> Result<bool, ServiceError>;
    
    // async fn is_credential_exists(&self, credential: &Credential) -> bool;
}
