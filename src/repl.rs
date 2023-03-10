use crate::chat;
use crate::completion;
use crate::models;
use crate::output::Output;
use regex::{Captures, Regex};
use std::env;
use std::io::{stdin, stdout, Write};
use text_colorizer::*;

fn string_to_vec(s: &str) -> Vec<String> {
    vec![String::from(s)]
}

fn parse_file(captures: Captures, print: bool) -> String {
    let args: Vec<&str> = captures[1].split(",").collect();
    if args.len() > 3 {
        eprintln!("Error {}", "Only maximum of 3 args are allowed".red());
    }
    let contents = std::fs::read_to_string(args[0]);
    let mut contents_to_use = String::from("");
    match contents {
        Ok(lines) => {
            for (index, line) in lines.lines().enumerate() {
                if args.len() > 1 {
                    let start = args[1].parse::<usize>().unwrap();
                    if index < start {
                        continue;
                    }
                }
                if args.len() > 2 {
                    let end = args[2].parse::<usize>().unwrap();
                    if index > end {
                        continue;
                    }
                }
                let new_line = format!("{}\n", line);
                contents_to_use.push_str(&new_line);
                if print {
                    println!("{}: {}", index, line.blue());
                }
            }
        }
        Err(e) => {
            eprintln!("Error {:?}", e);
        }
    }
    contents_to_use
}

fn exit_with_error(message: &str) {
    eprintln!("Error: {}", message.red());
    std::process::exit(1);
}

fn generate_messages_from_prompt(prompt: &str) -> Vec<chat::Message> {
    vec![chat::Message {
        role: Some(String::from("user")),
        content: Some(prompt.to_string()),
    }]
}

pub async fn run_repl() {
    let mut history = String::new();
    let mut input = String::new();
    let version: &str = env!("CARGO_PKG_VERSION");
    let re_cat_function = Regex::new(r"cat\(([^)]+)\)").unwrap();
    let re_include_function = Regex::new(r"include\(([^)]+)\)").unwrap();

    println!("gptrs {} \n", version);
    println!(
        "Type help() for information on commands, print() to print current query, exit() to quit"
    );
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable is not set, please set it before continuing \n Create an API Key here https://platform.openai.com/account/api-keys ... \n Exiting ... ");
            std::process::exit(1);
        }
    }
    loop {
        print!(">> ");
        stdout().flush().expect("Error flushing stdout");
        stdin().read_line(&mut input).expect("Error reading input");
        match input.trim() {
            "exit()" => break,
            "help()" => {
                println!("{}", "Instructions");
            }
            "chat()" => {
                let request = chat::ChatCreateCompletionParams {
                    max_tokens: Some(300),
                    model: Some(models::Models::Gpt35Turbo.name().to_string()),
                    messages: Some(generate_messages_from_prompt(&history)),
                    temperature: Some(0.7),
                };
                let output = chat::process_chat_prompt(request);
            }
            "complete()" => {
                let request = completion::CodeCompletionCreateParams {
                    max_tokens: 300,
                    model: models::Models::TextDavinci003.name().to_string(),
                    prompt: string_to_vec(&history),
                    temperature: 0.7,
                };
                let output = completion::process_completion_prompt(request).await;
                match output {
                    Ok(output) => output.to_cli(),
                    Err(e) => {
                        eprintln!("{}: {:?}", "Error".red(), e)
                    }
                }
                history = String::from("");
            }
            "clear()" => {
                history = String::from("");
            }
            "print()" => {
                println!("{}", history);
            }
            "ls()" => {
                //TODO: List Current Dir
            }
            "models()" => {
                //TODO: List Models, with Descriptions
            }
            _ => {
                // Parse Regex
                if let Some(captures) = re_cat_function.captures(&input) {
                    parse_file(captures, true);
                } else if let Some(captures) = re_include_function.captures(&input) {
                    let contents_to_use = parse_file(captures, false);
                    history.push_str(&contents_to_use);
                } else {
                    history.push_str(&input);
                }
            }
        }
        input.clear();
    }
}
