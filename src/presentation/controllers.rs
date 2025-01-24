use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json;

use crate::domain::Conversation;
use crate::application::services::{ChatService, ChatApi};

pub struct ChatController<T: ChatApi> {
    pub chat_service: ChatService<T>,
    pub conversation: Conversation,
    pub cache_file: String,
}

impl<T: ChatApi> ChatController<T> {
    pub fn new(
        mut chat_service: ChatService<T>,
        default_model: &str,
        cache_file: &str,
    ) -> Self {
        // Try to load from cache
        if let Ok(conv) = Self::load_conversation(cache_file) {
            // Update the ChatApi’s model if the loaded conversation 
            // used a different model
            let saved_model = conv.current_model.clone();
            chat_service.change_model(&mut Conversation::new(default_model), &saved_model);

            Self {
                chat_service,
                conversation: conv,
                cache_file: cache_file.to_owned(),
            }
        } else {
            // If no valid cache, start a fresh conversation
            Self {
                chat_service,
                conversation: Conversation::new(default_model),
                cache_file: cache_file.to_owned(),
            }
        }
    }

    /// Called when user presses ENTER on a typed message
    pub fn on_user_message(&mut self, user_input: &str) -> Result<()> {
        self.chat_service.send_and_receive(&mut self.conversation, user_input)?;
        self.save_conversation()?;
        Ok(())
    }

    /// Called when user changes the model
    pub fn on_change_model(&mut self, new_model: &str) -> Result<()> {
        self.chat_service.change_model(&mut self.conversation, new_model);
        self.save_conversation()?;
        Ok(())
    }

    /// Persist the conversation to disk as JSON
    pub fn save_conversation(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.conversation)?;
        fs::write(&self.cache_file, data)?;
        Ok(())
    }

    /// Load conversation from JSON file (if it exists)
    pub fn load_conversation(cache_file: &str) -> Result<Conversation> {
        if Path::new(cache_file).exists() {
            let data = fs::read_to_string(cache_file)?;
            let conv: Conversation = serde_json::from_str(&data)?;
            Ok(conv)
        } else {
            Err(anyhow::anyhow!("No cache file found"))
        }
    }
}
