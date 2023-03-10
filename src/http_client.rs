use crate::chat::ChatCreateCompletionParams;
use crate::completion::CodeCompletionCreateParams;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use reqwest::Result as ReqwestResult;
use serde_json::json;
use std::env;

//TODO: merge these into one client
pub async fn send_completion_request(
    request_base: CodeCompletionCreateParams,
) -> ReqwestResult<String> {
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(api_key) => {
            let mut request_builder = client.post("https://api.openai.com/v1/completions");
            let auth_header = format!("Bearer {}", api_key);
            request_builder = request_builder.header(AUTHORIZATION, auth_header);
            request_builder = request_builder.header(CONTENT_TYPE, "application/json");

            let json = json!({
                "model": request_base.model,
                "prompt": request_base.prompt,
                "temperature": request_base.temperature,
                "max_tokens": request_base.max_tokens,
            });
            let response = request_builder.json(&json).send().await?.text().await?;
            return Ok(response);
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable is not set, please set it before continuing");
            std::process::exit(1);
        }
    }
}

pub async fn send_chat_request(request_base: ChatCreateCompletionParams) -> ReqwestResult<String> {
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(api_key) => {
            let mut request_builder = client.post("https://api.openai.com/v1/completions");
            let auth_header = format!("Bearer {}", api_key);
            request_builder = request_builder.header(AUTHORIZATION, auth_header);
            request_builder = request_builder.header(CONTENT_TYPE, "application/json");

            let json = json!({
                "model": request_base.model,
                "messages": request_base.messages,
                "temperature": request_base.temperature,
                "max_tokens": request_base.max_tokens,
            });
            let response = request_builder.json(&json).send().await?.text().await?;
            return Ok(response);
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable is not set, please set it before continuing");
            std::process::exit(1);
        }
    }
}
