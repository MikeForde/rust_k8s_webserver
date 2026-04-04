use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub snowstorm_base: String,
    pub inactive_filtered: bool,
    pub app_admin_password: Option<String>,
    pub snowstorm_admin_username: Option<String>,
    pub snowstorm_admin_password: Option<String>,
}
