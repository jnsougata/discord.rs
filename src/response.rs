use serde_json::Value;

use crate::{
    state::AppState,
    interaction::Interaction,
    enums::InteractionCallbackType,
};


pub struct InteractionResponse {
    pub (crate) interaction: Interaction,
    pub (crate) state: AppState,
}

impl InteractionResponse {
    pub fn new( interaction: Interaction, state: AppState ) -> Self {
        Self {
            interaction,
            state,
        }
    }

    pub async fn send(&self, data: Value) -> Value {
        let state = self.state.clone();
        let payload = serde_json::json!({
            "type": InteractionCallbackType::ChannelMessageWithSource,
            "data": data
        }); 
        state.http.post_interaction_callback(
            self.interaction.token.clone(), 
            self.interaction.id.clone(), 
            payload
        ).await
    }
}