use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use reqwest::Result as ReqwestResult;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: String,
    index: usize,
    logprobs: Option<Logprobs>,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Logprobs {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

pub struct RequestBase {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub prompt: String,
}

pub async fn send_request(request_base: RequestBase) -> ReqwestResult<String> {
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
