use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub sender: String, // e.g. "User", "AI"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub messages: Vec<Message>,
    pub current_model: String,
}

impl Conversation {
    /// Start new conversation using the given model.
    pub fn new(model: &str) -> Self {
        Self {
            messages: Vec::new(),
            current_model: model.to_owned(),
        }
    }

    /// O(1) amortized insertion.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
