use std::collections::HashMap;
use crate::{
    http::HttpClient,
    command::ApplicationCommand,
};
use serde_json::Value;


#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub (crate) token: String,
    pub (crate) public_key: String,
    #[allow(dead_code)]
    pub (crate) application_id: String,
    pub (crate) commands: Vec<Value>,
    pub (crate) http: HttpClient,
    pub (crate) factory: HashMap<String, ApplicationCommand>,
}

impl AppState {
    pub fn new(
        public_key: String,
        token: String,
        application_id: String,
    ) -> Self {
        Self { 
            public_key, 
            token: token.clone(),
            application_id: application_id.clone(),
            factory: HashMap::new(),
            http: HttpClient::new(
                "discord.com/api/v10",
                application_id.clone(),
                token.clone()
            ),
            commands: Vec::new()
        }
    }
    pub async fn sync(&self) {
        let commands = serde_json::to_value(&self.commands).unwrap();
        self.http.register_commands(commands).await;
    }
}
