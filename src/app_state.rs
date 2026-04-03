use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub snowstorm_base: String,
}