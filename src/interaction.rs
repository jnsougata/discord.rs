use crate::enums::InteractionType;
use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub r#type: InteractionType,
    pub data: Option<Value>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub member: Option<Value>,
    pub user: Option<Value>,
    pub token: String,
    pub version: u8
}