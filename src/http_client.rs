use crate::chat::ChatCreateCompletionParams;
use crate::completion::CodeCompletionCreateParams;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use reqwest::Result as ReqwestResult;
use serde_json::json;
use serde_json::Value;
use std::env;

//TODO: merge these into one client
pub async fn send_completion_request(
    request_base: CodeCompletionCreateParams,
) -> ReqwestResult<String> {
    send_completion_base_request("https://api.openai.com", request_base).await
}

pub async fn send_completion_base_request(
    base_url: &str,
    request_base: CodeCompletionCreateParams,
) -> ReqwestResult<String> {
    let url = format!("{}{}", base_url, "/v1/completions");
    let request_json = json!({
        "model": request_base.model,
        "prompt": request_base.prompt,
        "temperature": request_base.temperature,
        "max_tokens": request_base.max_tokens,
    });
    send_base_request(&url, request_json).await
}

#[tokio::test]
async fn test_send_completion_base_request() {
    let mut server = mockito::Server::new_async().await;
    env::set_var("OPENAI_API_KEY", "test-token");

    let url = server.url();
    let request_base = CodeCompletionCreateParams {
        model: String::from("fake-model"),
        prompt: vec![String::from("hello!")],
        max_tokens: 3000,
        temperature: 0.7,
    };

    let mock = server
        .mock("POST", "/v1/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Authorization", "Bearer test-token")
        .with_body(r#"{"fake": "TODO: check request"}"#)
        .create_async()
        .await;

    _ = send_completion_base_request(&url, request_base).await;

    mock.assert_async().await;
}

//TODO: refactor this to make it easier to do mocking
pub async fn send_chat_request(request_base: ChatCreateCompletionParams) -> ReqwestResult<String> {
    send_chat_base_request("https://api.openai.com", request_base).await
}

pub async fn send_chat_base_request(
    base_url: &str,
    request_base: ChatCreateCompletionParams,
) -> ReqwestResult<String> {
    let url = format!("{}{}", base_url, "/v1/chat/completions");

    let request_json = json!({
        "model": request_base.model,
        "messages": request_base.messages,
        "temperature": request_base.temperature,
        "max_tokens": request_base.max_tokens,
    });
    send_base_request(&url, request_json).await
}

pub async fn send_base_request(url: &str, request_json: Value) -> ReqwestResult<String> {
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(api_key) => {
            let mut request_builder = client.post(url);
            let auth_header = format!("Bearer {}", api_key);
            request_builder = request_builder.header(AUTHORIZATION, auth_header);
            request_builder = request_builder.header(CONTENT_TYPE, "application/json");
            let response = request_builder
                .json(&request_json)
                .send()
                .await?
                .text()
                .await?;
            return Ok(response);
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable is not set, please set it before continuing");
            std::process::exit(1);
        }
    }
}

#[tokio::test]
async fn test_send_chat_base_request() {
    let mut server = mockito::Server::new_async().await;
    env::set_var("OPENAI_API_KEY", "test-token");

    let url = server.url();
    let request_base = ChatCreateCompletionParams {
        model: Some(String::from("fake-model")),
        messages: Some(vec![]),
        max_tokens: Some(3000),
        temperature: Some(0.7),
    };

    let mock = server
        .mock("POST", "/v1/chat/completions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Authorization", "Bearer test-token")
        .with_body(r#"{"fake": "TODO: check request"}"#)
        .create_async()
        .await;

    _ = send_chat_base_request(&url, request_base).await;

    mock.assert_async().await;
}
