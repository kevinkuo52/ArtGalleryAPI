use std::env;
//use lazy_static::lazy_static;
// lazy_static! {
//     pub static ref GOOGLE_CLIENT_ID: &'static str = std::env!("GOOGLE_CLIENT_ID");
// }
pub const GOOGLE_CLIENT_ID: &str = env!("GOOGLE_CLIENT_ID");
pub const GOOGLE_JWK_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";
pub const GOOGLE_AUTH_ISS: &str = "accounts.google.com";
pub const JWT_KEY: &[u8] = env!("JWT_KEY").as_bytes();



pub const USER_INDEX: &str = "user";
pub const CREDENTIAL_INDEX: &str = "credential";
pub const ARTWORK_INDEX: &str = "artwork";