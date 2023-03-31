use serde::{Serialize, Deserialize};

use super::chat_message::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i64>,
    pub stream: Option<bool>,
    pub max_tokens: Option<i64>,
}

