// use oauth2::{
//     AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
// };
// use std::env;
// use oauth2::basic::BasicClient;
// pub fn configure() -> BasicClient {
//     let google_client_id = ClientId::new(
//             env::var("GOOGLE_CLIENT_ID")
//                 .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
//         );
//         let google_client_secret = ClientSecret::new(
//             env::var("GOOGLE_CLIENT_SECRET")
//                 .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
//         );
//         let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
//             .expect("Invalid authorization endpoint URL");
//         let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
//             .expect("Invalid token endpoint URL");

//         // Set up the config for the Google OAuth2 process.
//         let client = BasicClient::new(
//             google_client_id,
//             Some(google_client_secret),
//             auth_url,
//             Some(token_url),
//         )
//         .set_redirect_url(
//             RedirectUrl::new("http://localhost:8080/auth".to_string())
//                 .expect("Invalid redirect URL"),
//         );
//         return client;
// }