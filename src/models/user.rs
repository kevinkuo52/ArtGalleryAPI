use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub username: String,
    pub art_works: Vec<String>,
    pub description: String,
    pub liked_art_works: Vec<String>,
    pub followers: Vec<String>,
    pub following: Vec<String>,
}





