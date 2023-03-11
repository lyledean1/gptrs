use crate::err::ApiError;
use crate::http_client;
use crate::output::Output;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::Result as SerdeResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeCompletionCreateParams {
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f64,
    pub prompt: Vec<String>,
    // pub stop: Option<Vec<String>>,
    // pub suffix: Option<String>,
    // pub stream: Option<bool>,
    // pub echo: Option<bool>,
    // pub n: Option<i32>,
    // pub top_p: Option<f64>,
    // pub logprobs: Option<i32>,
    // pub presence_penalty: Option<f64>,
    // pub frequency_penalty: Option<f64>,
    // pub best_of: Option<i32>,
    // pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CodeCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    text: String,
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

impl Output for CodeCompletionResponse {
    fn get_output(&self) -> String {
        let mut output = String::from("");
        for choice in self.choices.iter() {
            let lines = choice.text.lines();
            for line in lines {
                if line.trim().is_empty() {
                    continue; // ignore empty or whitespace-only lines
                }
                output.push_str(line);
                output.push_str("\n");
            }
        }
        output
    }
}

fn parse_completion_response(response: String) -> SerdeResult<CodeCompletionResponse> {
    match from_str(&response) {
        Ok(c) => return Ok(c),
        Err(e) => {
            return Err(e);
        }
    };
}

pub async fn process_completion_prompt(
    request_defaults: CodeCompletionCreateParams,
) -> Result<CodeCompletionResponse, ApiError> {
    //TODO: Readd Language
    let result = http_client::send_completion_request(request_defaults).await;
    match result {
        Ok(response) => match parse_completion_response(response) {
            Ok(completion) => {
                return Ok(completion);
            }
            Err(e) => {
                return Err(ApiError::new(&e.to_string()));
            }
        },
        Err(e) => {
            return Err(ApiError::new(&e.to_string()));
        }
    }
}
