mod completion;
use std;
use std::fs;
use text_colorizer::*;
use std::env;
use serde_json::Result as SerdeResult;
use serde_json::{from_str};
use clap::{arg, Command};

fn cli() -> Command {
    Command::new("gptrs")
    .subcommand( Command::new("completion")
    .args([
        arg!(--prompt <PROMPT> "Prompt to enter in chatgpt").group("completion"),
        arg!(--file <FILE> "Include a file to get a response from chatgpt").group("completion"),
    ]))
}

fn parse_completion_response(response: String) -> SerdeResult<completion::CodeCompletion> {
    match from_str(&response) {
        Ok(c) => {
            return Ok(c)
        },
        Err(e) => {
            return Err(e);
        }
    };
}

async fn parse_completion_file(api_key: &str, path: &str) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    process_completion_prompt(&contents.as_str(), api_key).await
}

async fn process_completion_prompt(prompt: &str, api_key: &str) {
    //TODO: Readd Language
    let result = completion::send_request(&prompt, &api_key).await;
    println!("");
    match result {
        Ok(response) => {
            match parse_completion_response(response) {
                Ok(completion) => {
                    println!("{}", &prompt.bold());
                    println!("");
                    println!("---Response---");
                    println!("");
                    for choice in completion.choices.iter() {
                        let lines = choice.text.lines();
                        for line in lines {
                            if line.trim().is_empty() {
                                continue; // ignore empty or whitespace-only lines
                            }
                            println!("{}", line.yellow())
                        }
                    }
                    println!("");
                },
                Err(e) => {
                    eprintln!("Error {}", e);
                }
            }
        },
        Err(error) => {
            eprintln!("Error: {}", error);
        },
    }
}


#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(_) => {},
        Err(_) => {
            eprintln!("{}: OPENAI_API_KEY environment variable is not set, please set it before continuing", "Error".red().bold());
            std::process::exit(1);
        },
    }

    let matches = cli().get_matches();

    if let Some(completion_matches) = matches.subcommand_matches("completion") {
        let prompt = completion_matches.get_one::<String>("prompt").map(|s| s.as_str());
        match prompt {
            Some(prompt) => {
                process_completion_prompt(&prompt, &api_key.clone().unwrap()).await;
            },
            None => {},
        }
        let file = completion_matches.get_one::<String>("file").map(|s| s.as_str());
        match file {
            Some(file) => {
                parse_completion_file(&api_key.clone().unwrap(), file).await;
            },
            None => {},
        }
    }
}

