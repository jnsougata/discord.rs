use axum::{
    Router,
    routing::get,
};
mod handler;
pub mod enums;
mod interaction;
mod app;
mod utils;

#[tokio::main]
async fn main() {

    let state = app::AppState{
        public_key: std::env::var("PUBLIC_KEY").unwrap().to_string(),
        token: std::env::var("DISCORD_TOKEN").unwrap() .to_string(),
        application_id: std::env::var("APPLICATION_ID").unwrap() .to_string(),
        interaction_path: "/interactions".to_string(),
    };
    app::App::new(state)
    .extend(
        Router::new().route("/", get(|| async { "Hello, World!" }))
    )
    .run(8080).await;
}
