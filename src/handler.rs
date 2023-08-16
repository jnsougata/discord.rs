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

    let interaction = serde_json::from_slice::<Interaction>(&data).unwrap();

    match interaction.kind {
        InteractionType::Ping => {
            return (StatusCode::OK, Json(json!({"type": InteractionCallbackType::Pong})));
        },
        InteractionType::ApplicationCommand => {
            let name = interaction.data.as_ref().unwrap()["name"].as_str().unwrap();
            let command = state.factory.get(name).unwrap().callback.clone();
            let (status, Json(json)) = command(interaction);
            return (status, Json(json));
        },
        InteractionType::MessageComponent => todo!(),
        InteractionType::ApplicationCommandAutocomplete => todo!(),
        InteractionType::ModalSubmit => todo!(),
    }
    
}
