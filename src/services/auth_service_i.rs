use async_trait::async_trait;
use crate::models::{
    auth::*,
    error::ServiceError,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService {
    async fn register(&self, req: &RegistrationReq, auth_type: &str) -> Result<(), ServiceError>;
    async fn verify_password(&self, req: &LoginReq) -> Result<String, ServiceError>;
    async fn authenticate(&self, token: &Token) -> Option<String>;
}