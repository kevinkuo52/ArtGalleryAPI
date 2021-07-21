use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub username: String,
    pub artworks: Vec<String>,
    pub description: String,
    pub liked_artworks: Vec<String>,
    pub followers: Vec<String>,
    pub following: Vec<String>,
}





