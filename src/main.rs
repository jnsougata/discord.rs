use axum::{
    Router,
    routing::get,
    routing::post,
};
mod handler;
pub mod enums;
mod interaction;
mod state;

#[tokio::main]
async fn main() {

    let state = state::AppState{
        public_key: std::env::var("PUBLIC_KEY").unwrap().to_string(),
        token: std::env::var("DISCORD_TOKEN").unwrap() .to_string(),
        application_id: std::env::var("APPLICATION_ID").unwrap() .to_string(),
        path: "/interactions".to_string(),
        port: 8080,
    };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(&state.path.clone(), post(handler::handler).with_state(state));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

