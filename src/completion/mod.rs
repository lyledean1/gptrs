use serde::{Serialize, Deserialize};
use serde_json::json;

use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use reqwest::Result as ReqwestResult;

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

pub async fn send_request(prompt: &str, api_key: &str) -> ReqwestResult<String> {
    let client = Client::new();

    let mut request_builder = client.post("https://api.openai.com/v1/completions");
    let auth_header = format!("Bearer {}", api_key);
    request_builder = request_builder.header(AUTHORIZATION, auth_header);
    request_builder = request_builder.header(CONTENT_TYPE, "application/json");


    let json = json!({
        "model": "text-davinci-002",
        "prompt": prompt,
        "temperature": 0.7,
        "max_tokens": 300
    });

    let response = request_builder
        .json(&json)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}