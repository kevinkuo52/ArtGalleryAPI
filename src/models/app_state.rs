use oauth2::basic::BasicClient;

pub struct AppState {
    pub oauth: BasicClient,
}