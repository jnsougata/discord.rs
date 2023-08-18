use serde_json::Value;
use serde::Deserialize;
use crate::{ enums::InteractionType, response::InteractionResponse, state::AppState };


#[derive(Deserialize, Clone)]
#[non_exhaustive]
pub struct InteractionData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub resolved: Option<Value>,
    pub options: Option<Vec<Value>>,
    pub custom_id: Option<String>,
    pub component_type: Option<u8>,
    pub values: Option<Vec<String>>,
    pub target_id: Option<String>,
    pub guild_id: Option<String>,
}


#[derive(Deserialize, Clone)]
#[non_exhaustive]
pub struct Interaction {
    pub id: String,
    pub token: String,
    pub version: u8,
    pub application_id: String,
    pub channel_id: Option<String>,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<String>,
    pub member: Option<Value>,
    pub user: Option<Value>,
    #[serde(skip_deserializing)]
    pub state: Option<AppState>,
}


impl Interaction {
    pub fn response(&self) -> InteractionResponse {
        InteractionResponse::new(self.clone(), self.state.clone().unwrap())
    }
}