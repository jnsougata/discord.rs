use std::sync::{Arc};
use axum::{
    Router,
    routing::get,
};
use crate::interaction::Interaction;

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
        options: None,
        dm_permissions: None,
        nsfw: None,
        default_member_permissions: None,
        callback: Arc::new(Box::new(|_interaction: Interaction| {
            (
                axum::http::StatusCode::OK,
                axum::Json(
                    serde_json::json!(
                        {
                            "type": enums::InteractionCallbackType::ChannelMessageWithSource,
                            "data": {
                                "content": "Pong!"
                            }
                        }
                    )
                )
            )
        })),
    };

    app::App::new(
        std::env::var("PUBLIC_KEY").unwrap().to_string(),
        std::env::var("DISCORD_TOKEN").unwrap() .to_string(),
        std::env::var("APPLICATION_ID").unwrap() .to_string(),
        "/interactions".to_string(),
    )
        .extend(
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
        )
        .add_command(cmd)
        .run(8080).await;
}
