use axum::{
    extract::{Json, State}, 
    http::{header::HeaderMap, StatusCode},
};
use serde_json;
use serde_json::{Value, json};
use ed25519_dalek::{Signature, SigningKey};
use crate::enums::InteractionType;
use crate::interaction::Interaction;
use crate::state::AppState;


pub (crate) async fn handler(
    headers: HeaderMap,
    State(state): State<AppState>, 
    payload: Json<Value>
) -> (StatusCode, Json<Value>){
    let signature = headers.get("X-Signature-Ed25519").unwrap();
    let timestamp = headers.get("X-Signature-Timestamp").unwrap();
    let signature_bytes = signature.as_bytes();
    let timestamp_bytes = timestamp.as_bytes();
    let public_key_bytes: &[u8; 32] = state.public_key.as_bytes().try_into().unwrap();
    let mut message = Vec::new();
    message.extend_from_slice(timestamp_bytes);
    message.extend_from_slice(payload.to_string().as_bytes());
    let signature = Signature::from_slice(signature_bytes).unwrap();
    let public_key = SigningKey::from_bytes(&public_key_bytes);
    let verified = public_key.verify(&message, &signature);
    if !verified.is_ok() {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "BadSignature"})));
    }
    let interaction: Interaction = serde_json::from_value(payload.0).unwrap();

    match interaction.r#type {
        InteractionType::Ping => {
            return (StatusCode::OK, Json(json!({"type": InteractionType::Ping})));
        },
        InteractionType::ApplicationCommand => {
            return (
                StatusCode::OK, 
                Json(json!({
                    "type": 4,
                    "data": {
                        "content": "Hello, World!"
                    }
                }))
            );
        },
        InteractionType::MessageComponent => todo!(),
        InteractionType::ApplicationCommandAutocomplete => todo!(),
        InteractionType::ModalSubmit => todo!(),
    }
    
}
