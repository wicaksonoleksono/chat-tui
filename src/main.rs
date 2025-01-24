mod domain;
mod application;
mod infrastructure;
mod presentation;

use anyhow::Result;
use crate::infrastructure::http_client::OllamaClient;
use crate::application::services::ChatService;
use crate::presentation::{
    controllers::ChatController,
    tui::TuiApp,
};

fn main() -> Result<()> {
    // 1) Create Ollama client with base_url & default model
    //    If you want to connect to http:// instead of https://, or a different port, adjust here.
    let ollama_client = OllamaClient::new("https://localhost:11434", "nemotron");

    // 2) Create our ChatService
    let chat_service = ChatService::new(ollama_client);

    // 3) Create a controller with caching
    //    We’ll store conversation data in a local JSON file named conversation_cache.json
    let controller = ChatController::new(chat_service, "nemotron", "conversation_cache.json");

    let mut tui_app = TuiApp::new(controller);
    tui_app.run()?;

    Ok(())
}
