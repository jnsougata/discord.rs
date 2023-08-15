use crate::http::HttpClient;
use crate::command::ApplicationCommand;
use serde_json;


#[derive(Clone, Debug)]
pub struct AppState {
    pub public_key: String,
    pub token: String,
    pub application_id: String,
    pub interaction_path: String,
    commands: Vec<ApplicationCommand>,
    http: HttpClient,
}

impl AppState {
    pub fn new(public_key: String, token: String, application_id: String, interaction_path: String) -> Self {
        Self {
            public_key,
            token: token.to_owned(),
            application_id: application_id.to_owned(),
            interaction_path,
            commands: Vec::new(),
            http: HttpClient::new("discord.com/api/v10",application_id, token),
        }
    }

    pub fn add_command(&mut self, command: ApplicationCommand) {
        self.commands.push(command);
    }

    pub async fn sync(self) {
        let commands = serde_json::to_value(&self.commands).unwrap();
        println!("{:?}", commands);
        println!("{}", self.http.register_commands(commands).await)
    }
}
