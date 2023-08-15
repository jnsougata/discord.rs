use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommand {
    #[serde(rename = "type")]
    pub kind: u8,
    pub name: String,
    pub description: String,
}

