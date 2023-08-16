use serde::Serialize;
use crate::{
    enums::{ApplicationCommandType, ApplicationCommandOptionType},
};


#[derive(Debug, Clone, Serialize)]
pub struct ApplicationCommandChoice {
    pub name: String,
    pub value: Vec<u8>,
}

impl ApplicationCommandChoice {
    pub fn from<T>( name: String, value: T ) -> Self
    where T: Into<Vec<u8>> {
        Self {
            name,
            value: value.into(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Default)]
pub struct ApplicationCommandOption {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: ApplicationCommandOptionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ApplicationCommandChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>
}

impl ApplicationCommandOption {
    pub fn string(
        name: &str,
        description: &str,
        required: Option<bool>,
        max_length: Option<i32>,
        min_length: Option<i32>,
        choices: Option<Vec<ApplicationCommandChoice>>,
        autocomplete: Option<bool>
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required,
            max_length,
            min_length,
            choices,
            autocomplete,
            max_value: None,
            min_value: None,
            channel_types: None,
            kind: ApplicationCommandOptionType::String,
        }
    }

    pub fn integer(
        name: &str,
        description: &str,
        required: Option<bool>,
        max_value: Option<i32>,
        min_value: Option<i32>,
        choices: Option<Vec<ApplicationCommandChoice>>,
        autocomplete: Option<bool>
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required,
            max_value,
            min_value,
            choices,
            autocomplete,
            max_length: None,
            min_length: None,
            channel_types: None,
            kind: ApplicationCommandOptionType::Integer,
        }
    }

    pub fn number<T>(
        name: &str,
        description: &str,
        required: Option<bool>,
        max_value: Option<T>,
        min_value: Option<T>,
        choices: Option<Vec<ApplicationCommandChoice>>,
        autocomplete: Option<bool>
    ) -> Self
    where T: Into<i32> {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required,
            max_value: max_value.map(|v| v.into()),
            min_value: min_value.map(|v| v.into()),
            choices,
            autocomplete,
            max_length: None,
            min_length: None,
            channel_types: None,
            kind: ApplicationCommandOptionType::Number,
        }
    }

    pub fn channel(
        name: &str,
        description: &str,
        required: Option<bool>,
        channel_types: Option<Vec<u8>>
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            required,
            channel_types,
            autocomplete: None,
            max_length: None,
            min_length: None,
            max_value: None,
            min_value: None,
            choices: None,
            kind: ApplicationCommandOptionType::Channel
        }
    }

    pub fn others (
        name: &str,
        description: &str,
        kind: ApplicationCommandOptionType,
        required: Option<bool>,
    ) -> Self
    {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            kind,
            required,
            choices: None,
            autocomplete: None,
            max_length: None,
            min_length: None,
            max_value: None,
            min_value: None,
            channel_types: None
        }
    }
}


#[derive(Debug, Clone, Serialize, Default)]
pub struct ApplicationCommand {
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,
}
