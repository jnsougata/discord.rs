#[derive(Clone)]
pub struct AppState {
    pub public_key: String,
    pub token: String,
    pub application_id: String,
    pub path: String,
    pub port: u16,
}