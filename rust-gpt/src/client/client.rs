use crate::{constants, model::{chat_request, chat_response}, error};

pub struct OpenAIClient {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> OpenAIClient {
        OpenAIClient {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_chat_completion(&self, request: &chat_request::ChatRequest) -> Result<chat_response::ChatResponse, error::RustGptError> {
        let response = self.client
            .post(constants::CHAT_URI)
            .header("Context-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(request)
            .send()
            .await?;

        let response = response.json::<chat_response::ChatResponse>().await?;
        Ok(response)
    }
}