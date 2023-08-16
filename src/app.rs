use std::net::SocketAddr;

use axum::{
    Router,
    routing::post,
};

use crate::{
    handler::handler,
    state::AppState,
    command::ApplicationCommand,
};

pub struct App {
    pub token: String,
    pub application_id: String,
    pub interaction_path: String,
    state: AppState,
    router: Router,
}

impl App {
    pub fn new(
        public_key: String,
        token: String,
        application_id: String,
        interaction_path: String
    ) -> Self {
        Self {
            token: token.clone(),
            application_id: application_id.clone(),
            interaction_path,
            state: AppState::new(
                public_key,
                token.clone(),
                application_id.clone(),
            ),
            router: Router::new(),
        }
    }

    pub fn extend(mut self, router: Router) -> Self {
        self.router = self.router.nest("/", router);
        self
    }

    pub fn add_command(mut self, command: ApplicationCommand) -> Self{
        self.state.commands.push(serde_json::to_value(&command).unwrap());
        self.state.factory.insert(command.name.clone(), command);
        self
    }

    pub async fn run(&self, port: u16,) {
        let route = self.router.clone()
            .route(&self.interaction_path, post(handler).with_state(self.state.clone()));
        axum::Server::bind(&format!("0.0.0.0:{}", port).to_string().parse::<SocketAddr>().unwrap())
        .serve(route.into_make_service())
        .await
        .unwrap();
    }
}
    