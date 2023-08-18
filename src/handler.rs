use hyper;
use axum::{
    extract::{Json, State, RawBody}, 
    http::{header::HeaderMap, StatusCode},
};
use serde_json;
use serde_json::{Value, json};
use crate::{
    state::AppState,
    utils::verify_signature,
    interaction::Interaction,
    enums::{InteractionType, InteractionCallbackType},
};



pub (crate) async fn handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    RawBody(body): RawBody,
) -> (StatusCode, Json<Value>){
    let data = hyper::body::to_bytes(body).await.unwrap();
    let public_key_hex = state.public_key.as_str();
    let signature_hex = headers.get("X-Signature-Ed25519").unwrap().to_str().unwrap();
    let timestamp = headers.get("X-Signature-Timestamp").unwrap().as_bytes();

    let result = verify_signature(signature_hex, public_key_hex, &data, timestamp);
   
    if !result{
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "BadSignature"})));
    }

    let mut interaction = serde_json::from_slice::<Interaction>(&data).unwrap();
    interaction.state = Some(state.clone());

    match interaction.kind {
        InteractionType::Ping => {
            return (StatusCode::OK, Json(json!({"type": InteractionCallbackType::Pong})));
        },
        InteractionType::ApplicationCommand => {
            let data = interaction.data.as_ref().unwrap().clone();
            let command_ref_key = format!("{}:{}", data.name, data.kind as u8);
            println!("{}", command_ref_key);
            _ = state.factory.get(&command_ref_key).unwrap().invoke(interaction.clone());
            return (axum::http::StatusCode::OK, Json(json!({"message": "sent"})));
        },
        InteractionType::MessageComponent => todo!(),
        InteractionType::ApplicationCommandAutocomplete => todo!(),
        InteractionType::ModalSubmit => todo!(),
    }
}
