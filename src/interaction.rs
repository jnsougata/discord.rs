use serde_json::Value;
use serde::Deserialize;
use crate::enums::InteractionType;

#[derive(Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct Interaction {
    pub id: String,
    pub token: String,
    pub version: u8,
    pub application_id: String,
    pub channel_id: Option<String>,

    #[serde(rename = "type")]
    pub kind: InteractionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Value>
}
