use crate::{model::{chat_completions, models}, error};

const CHAT_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODELS_URL: &str = "https://api.openai.com/v1/models";

pub struct OGptAsyncClient {
    api_key: String,
    client: reqwest::Client,
}

impl OGptAsyncClient {
    pub fn new(api_key: String) -> OGptAsyncClient {
        OGptAsyncClient {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn chat_completion_async(&self, request: &chat_completions::ChatCompletionsRequest) -> Result<chat_completions::ChatCompletionsResponse, error::OGptError> {
        let response = self.client
            .post(CHAT_COMPLETIONS_URL)
            .header("Context-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(request)
            .send()
            .await?;

        let response = response.json::<chat_completions::ChatCompletionsResponse>().await?;
        Ok(response)
    }

    pub async fn models_async(&self) -> Result<models::ModelsResponse, error::OGptError> {
        let response = self.client
            .get(MODELS_URL)
            .header("Context-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .send()
            .await?;

        let response = response.json::<models::ModelsResponse>().await?;
        Ok(response)
    }
}

pub struct OGptSyncClient {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl OGptSyncClient {
    pub fn new(api_key: String) -> OGptSyncClient {
        OGptSyncClient {
            api_key,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub async fn chat_completion_sync(&self, request: &chat_completions::ChatCompletionsRequest) -> Result<chat_completions::ChatCompletionsResponse, error::OGptError> {
        let response = self.client
            .post(CHAT_COMPLETIONS_URL)
            .header("Context-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(request)
            .send()?;

        let response = response.json::<chat_completions::ChatCompletionsResponse>()?;
        Ok(response)
    }

    pub async fn models_sync(&self) -> Result<models::ModelsResponse, error::OGptError> {
        let response = self.client
            .get(MODELS_URL)
            .header("Context-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .send()?;

        let response = response.json::<models::ModelsResponse>()?;
        Ok(response)
    }
}