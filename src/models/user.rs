use serde::{Serialize, Deserialize};

// #[derive(Deserialize)]
// pub struct AuthReq {
//     pub code: String,
//     pub state: String,
//     pub scope: String,
// }
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub email: String,
    pub username: String,
}





