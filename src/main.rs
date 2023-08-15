use axum::{
    Router,
    routing::get,
};
mod handler;
pub mod enums;
mod interaction;
pub mod app;
pub mod state;
mod utils;
mod http;
mod command;

#[tokio::main]
async fn main() {

    let mut state = state::AppState::new(
        std::env::var("PUBLIC_KEY").unwrap().to_string(),
        std::env::var("DISCORD_TOKEN").unwrap() .to_string(),
        std::env::var("APPLICATION_ID").unwrap() .to_string(),
        "/interactions".to_string(),
    );
    state.add_command(command::ApplicationCommand{
        kind: 1,
        name: "ping".to_string(),
        description: "pong".to_string(),
    });
    state.clone().sync().await;
    app::App::new(state)
    .extend(
        Router::new().route("/", get(|| async { "Hello, World!" }))
    )
    .run(8080).await;
}
