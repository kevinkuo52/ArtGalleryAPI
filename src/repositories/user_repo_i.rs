use async_trait::async_trait;
use crate::models::{
    user::{User,},
    elastic_res::{DocES}
};
use crate::models::error::ServiceError;
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepo  {
    async fn create_user(&self, id: &String, username: &String) -> Result<(), ServiceError>;
    async fn get_user(self: &Self, id: &String) -> Result<DocES<User>, ServiceError>;
}
