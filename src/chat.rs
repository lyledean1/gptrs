use crate::err::ApiError;
use crate::http_client;
use crate::output::Output;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::Result as SerdeResult;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GptChat {
    messages: Vec<Message>,
}

pub trait History {
    fn new() -> GptChat;
    fn add(&mut self, message: Message);
    fn pop(&mut self);
    fn get_all(&self) -> Vec<Message>;
    fn flush(&mut self);
}

impl History for GptChat {
    fn new() -> GptChat {
        GptChat { messages: vec![] }
    }

    fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn pop(&mut self) {
        self.messages.pop();
    }

    fn get_all(&self) -> Vec<Message> {
        self.messages.clone()
    }

    fn flush(&mut self) {
        self.messages = vec![];
    }
}

// Based off create chat completion
// See API reference here https://platform.openai.com/docs/api-reference/chat/create
#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCreateCompletionParams {
    pub model: Option<String>,
    pub messages: Option<Vec<Message>>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i32>,
    //TODO: readd
    // stop: Option<Vec<String>>,
    // stream: Option<bool>,
    // n: Option<i32>,
    // top_n: Option<f64>,
    // presence_penalty: Option<f64>,
    // frequency_penalty: Option<f64>,
    // user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCreateCompletionResponse {
    id: String,
    object: Option<String>,
    created_at: Option<i64>,
    choices: Option<Vec<Choice>>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    message: Option<Message>,
    index: Option<i32>,
    logprobs: Option<i32>,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: Option<i32>,
    completion_tokens: Option<i32>,
    total_tokens: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub message: String,
    pub r#type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    ChatCreateCompletion(ChatCreateCompletionResponse),
    Error(ErrorResponse),
}


impl Output for ChatCreateCompletionResponse {
    fn get_output(&self) -> String {
        let mut output = String::from("");
        for choice in self.choices.iter() {
            for message in choice.iter() {
                let lines = &message.message;
                let some_lines = lines;
                match some_lines {
                    Some(some_lines) => {
                        for line in &some_lines.content {
                            if line.trim().is_empty() {
                                continue; // ignore empty or whitespace-only lines
                            }
                            output.push_str(&line.to_string());
                            output.push_str("\n");
                        }
                    }
                    None => {}
                }
            }
        }
        output
    }
}

impl Output for ErrorResponse {
    fn get_output(&self) -> String {
        let output = format!("{:?}", self);
        String::from(output)
    }
}

pub trait MessageHistory {
    fn save_messages(&self, history: &mut GptChat);
}

impl MessageHistory for ChatCreateCompletionResponse {
    fn save_messages(&self, history: &mut GptChat) {
        for choice in self.choices.iter() {
            for message in choice.iter() {
                let lines = &message.message;
                let some_lines = lines;
                match some_lines {
                    Some(some_lines) => history.add(Message {
                        role: some_lines.role.clone(),
                        content: some_lines.content.clone(),
                    }),
                    None => {}
                }
            }
        }
    }
}

fn parse_chat_response(response: String) -> SerdeResult<Response> {
    match from_str(&response) {
        //TODO: make sure decoding is happining here properly
        Ok(c) => return Ok(c),
        Err(e) => {
            return Err(e);
        }
    };
}

pub async fn process_chat_prompt(
    request_defaults: ChatCreateCompletionParams,
) -> Result<ChatCreateCompletionResponse, ApiError> {
    //TODO: Readd Language
    let result = http_client::send_chat_request(request_defaults).await;
    match result {
        Ok(response) => match parse_chat_response(response) {
            Ok(completion) => {
                match completion {
                    Response::ChatCreateCompletion(r)  => {
                        return Ok(r);
                    }
                    Response::Error(e) => {
                        return Err(ApiError::new(&e.get_output()));
                    }

                }
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
