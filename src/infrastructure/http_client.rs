use anyhow::{Result, anyhow};
use reqwest::blocking::Client;
use crate::application::services::ChatApi;

pub struct OllamaClient {
    http: Client,
    base_url: String,
    model: String,
}

impl OllamaClient {
    /// Create new client. 
    /// `model` is the default model, e.g. "nemotron".
    pub fn new(base_url: &str, model: &str) -> Self {
        let client = Client::builder()
            .user_agent("my_chat_tui/0.1.0")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        Self {
            http: client,
            base_url: base_url.to_owned(),
            model: model.to_owned(),
        }
    }
}

impl ChatApi for OllamaClient {
    /// Send a message to the current model (or override with the param).
    fn send_message(&self, prompt: &str, model: &str) -> Result<String> {
        // We’ll use the input `model` rather than self.model, 
        // so it’s always consistent with the conversation.
        let url = format!("{}/generate", self.base_url);

        #[derive(serde::Serialize)]
        struct OllamaRequest<'a> {
            prompt: &'a str,
            model: &'a str,
        }

        #[derive(serde::Deserialize)]
        struct OllamaResponse {
            response: String,
        }

        let req_body = OllamaRequest {
            prompt,
            model,
        };

        // Attempt the POST request
        let resp = self.http
            .post(&url)
            .json(&req_body)
            .send()
            .map_err(|e| anyhow!("Error connecting to Ollama: {}", e))?
            .error_for_status()
            .map_err(|e| anyhow!("Ollama server error: {}", e))?
            .json::<OllamaResponse>()
            .map_err(|e| anyhow!("Invalid JSON from Ollama: {}", e))?;

        Ok(resp.response)
    }

    /// Update the model locally, so we know which one we’re using.
    fn set_model(&mut self, model: &str) {
        self.model = model.to_owned();
    }
}
