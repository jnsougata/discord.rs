use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit
}