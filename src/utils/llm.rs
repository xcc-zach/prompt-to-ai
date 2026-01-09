use crate::config::current_model_config;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Response error: {0}")]
    ResponseError(String),
}
#[derive(Deserialize, Serialize)]
pub struct Message {
    role: String,
    content: String,
}
type ChatHistory = Vec<Message>;
pub fn get_response(messages: ChatHistory) -> Result<String, Box<dyn Error>> {
    let model_config = current_model_config()?;
    let target_url = format!(
        "{}/chat/completions",
        model_config.base_url.trim_end_matches('/')
    );
    let api_key = model_config.api_key;
    let req = ChatRequest {
        model: model_config.model,
        messages,
    };
    let response: ChatResponse = Client::new()
        .post(target_url)
        .bearer_auth(api_key)
        .json(&req)
        .send()?
        .json()?;
    let content = response
        .choices
        .into_iter()
        .next()
        .ok_or(LLMError::ResponseError("empty choices".to_string()))?
        .message
        .content;
    Ok(content)
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: ChatHistory,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}
#[derive(Deserialize)]
struct Choice {
    message: Message,
}
