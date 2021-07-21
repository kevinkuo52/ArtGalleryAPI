use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Artwork{
    pub img_url: String,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CreateArtworkReq{
    pub img_url: String,
    pub title: String,
    pub description: String,
    pub tags: String,
}

impl From<CreateArtworkReq> for Artwork {
    fn from(req: CreateArtworkReq, user_id: &String) -> Self{
        Self {
            img_url: req.img_url,
            title: req.title,
            description: req.description,
            tags: req.tags,
            user: user_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CreateArtworkRes{
    pub artwork_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArtworkRes {
    pub id: String,
    pub img_url: String,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArtworkListRes{
    artworks: Vec<ArtworkRes>,
}

