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

    let cmd = command::ApplicationCommand{
        kind: enums::ApplicationCommandType::Slash,
        name: "ping".to_string(),
        description: "ping the bot".to_string(),
        ..Default::default()
    };

    println!("{:?}", serde_json::to_string(&cmd).unwrap());

    let mut state = state::AppState::new(
        std::env::var("PUBLIC_KEY").unwrap().to_string(),
        std::env::var("DISCORD_TOKEN").unwrap() .to_string(),
        std::env::var("APPLICATION_ID").unwrap() .to_string(),
        "/interactions".to_string(),
    );
    state.add_command(cmd);
    state.clone().sync().await;
    app::App::new(state.clone())
    .extend(
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/sync", get(|| async move {
                state.clone().sync().await;
                "synced"
            })) 
    )
    .run(8080).await;
}
