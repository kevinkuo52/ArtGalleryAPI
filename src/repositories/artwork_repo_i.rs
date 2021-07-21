use async_trait::async_trait;
use crate::models::{
    artwork::{
        Artwork,
    },
    elastic_res::{DocES}
};
use crate::models::error::ServiceError;
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ArtworkRepo  {
    async fn create_artwork(&self, artwork: &Artwork) -> Result<String, ServiceError>;
    async fn get_artwork(&self, id: &String) ->Result<DocES<Artwork>, ServiceError>;
}
