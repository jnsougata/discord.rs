use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit
}

impl TryFrom<u8> for InteractionType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(InteractionType::Ping),
            2 => Ok(InteractionType::ApplicationCommand),
            3 => Ok(InteractionType::MessageComponent),
            4 => Ok(InteractionType::ApplicationCommandAutocomplete),
            5 => Ok(InteractionType::ModalSubmit),
            _ => Err(())
        }
    }
}    

#[derive(Deserialize, Serialize, Debug)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource,
    DeferredUpdateMessage,
    UpdateMessage,
    ApplicationCommandAutocompleteResult,
    Modal,
}