use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Artwork{
    pub user_id: String,
    pub img_url: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArtworkRes {
    pub id: String,
    pub user_id: String,
    pub img_url: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Artwork {
    pub fn to_artwork_res(&self, artwork_id: &String) -> ArtworkRes{
        ArtworkRes {
            id: artwork_id.to_string(),
            user_id: self.user_id.clone(),
            img_url: self.img_url.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CreateArtworkReq{
    pub img_url: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl CreateArtworkReq {
    pub fn to_model(&self, user_id: &String) -> Artwork{
        Artwork{
            user_id: user_id.to_string(),
            img_url: self.img_url.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CreateArtworkRes{
    pub artwork_id: String,
}




#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArtworkListRes{
    artworks: Vec<ArtworkRes>,
}

