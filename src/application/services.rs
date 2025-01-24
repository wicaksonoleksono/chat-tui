use anyhow::Result;
use crate::domain::{Conversation, Message};

/// The external LLM interface.
pub trait ChatApi {
    fn send_message(&self, prompt: &str, model: &str) -> Result<String>;
    fn set_model(&mut self, model: &str);
}

/// The core chat service that handles domain logic and calls ChatApi.
pub struct ChatService<T: ChatApi> {
    api_client: T,
}

impl<T: ChatApi> ChatService<T> {
    pub fn new(api_client: T) -> Self {
        Self { api_client }
    }
    pub fn send_and_receive(
        &mut self,
        conversation: &mut Conversation,
        user_msg: &str
    ) -> Result<()> {
        // 1. Add user message
        let user_message = Message {
            content: user_msg.to_owned(),
            sender: "User".to_owned(),
        };
        conversation.add_message(user_message);

        // 2. Ask the external AI
        let ai_response = self.api_client.send_message(user_msg, &conversation.current_model)?;

        // 3. Add AI message
        let ai_message = Message {
            content: ai_response,
            sender: "AI".to_owned(),
        };
        conversation.add_message(ai_message);

        Ok(())
    }

    /// Switch to a new model.
    pub fn change_model(&mut self, conversation: &mut Conversation, new_model: &str) {
        conversation.current_model = new_model.to_string();
        self.api_client.set_model(new_model);
    }
}
