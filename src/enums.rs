use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit
}

impl<'de> Deserialize<'de> for InteractionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: u32 = Deserialize::deserialize(deserializer)?;
        match value {
            1 => Ok(InteractionType::Ping),
            2 => Ok(InteractionType::ApplicationCommand),
            3 => Ok(InteractionType::MessageComponent),
            4 => Ok(InteractionType::ApplicationCommandAutocomplete),
            5 => Ok(InteractionType::ModalSubmit),
            _ => Err(serde::de::Error::custom(format!(
                "Unsupported value for InteractionType: {}",
                value
            ))),
        }
    }
}
   

#[derive(Debug, Copy, Clone)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource,
    DeferredUpdateMessage,
    UpdateMessage,
    ApplicationCommandAutocompleteResult,
    Modal,
}

impl Serialize for InteractionCallbackType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum ApplicationCommandType {
    #[default]
    Slash = 1,
    User,
    Message,

}

impl Serialize for ApplicationCommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    #[default]
    String=3,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Number,
    Attachment,
}

impl Serialize for ApplicationCommandOptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}