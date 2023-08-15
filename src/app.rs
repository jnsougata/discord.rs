use std::net::SocketAddr;

use crate::handler;
use axum::{
    Router,
    routing::post,
};
use crate::state::AppState;


pub struct App {
    state: AppState,
    router: Router,
}

impl App {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            router: Router::new(),
        }
    }

    pub fn extend(mut self, router: Router) -> Self {
        self.router = self.router.nest("/", router);
        self
    }

    pub async fn run(&self, port: u16,) {
        let route = self.router.clone()
            .route(&self.state.interaction_path, post(handler::handler).with_state(self.state.clone()));
        axum::Server::bind(&format!("0.0.0.0:{}", port).to_string().parse::<SocketAddr>().unwrap())
        .serve(route.into_make_service())
        .await
        .unwrap();
    }
}
    