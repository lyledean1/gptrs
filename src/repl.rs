use crate::chat;
use crate::chat::History;
use crate::chat::MessageHistory;
use crate::completion;
use crate::models;
use crate::output::Output;
use regex::{Captures, Regex};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use std::env;
use std::process::Command;
use text_colorizer::*;

fn string_to_vec(s: &str) -> Vec<String> {
    vec![String::from(s)]
}

fn get_max_tokens(captures: Captures) -> i32 {
    let args: Vec<&str> = captures[1].split(",").collect();
    args[0].parse::<i32>().unwrap()
}

fn parse_terminal_command(captures: Captures) -> String {
    let command_parts: Vec<&str> = captures[1].trim().split(' ').collect();

    let folder = env::current_dir().unwrap(); // get the current directory as a Path object

    println!(
        "Executing command {:?} in dir {:?}",
        captures[1].to_string(),
        folder.to_str()
    );

    let child = Command::new(command_parts[0])
        .args(&command_parts[1..])
        .current_dir(folder) // specify the current directory as the working directory
        .spawn()
        .expect("failed to execute process");

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());

    let command_output = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Output: {}", command_output);
    command_output
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

fn generate_message_from_prompt(prompt: &str) -> chat::Message {
    chat::Message {
        role: Some(String::from("user")),
        content: Some(prompt.to_string()),
    }
}

pub async fn run_repl() {
    let mut history = String::new();
    let mut chat_history = chat::GptChat::new();
    let mut max_tokens = 300;
    let version: &str = env!("CARGO_PKG_VERSION");

    //TODO: let this take strings in the form function("string", 1, 2) where 1 and 2 are optional arguments
    let re_cat_function = Regex::new(r"cat\(([^)]+)\)").unwrap();
    let re_max_tokens_function = Regex::new(r"max_tokens\(([^)]+)\)").unwrap();
    let re_file_function = Regex::new(r"file\(([^)]+)\)").unwrap();
    let re_shell_function = Regex::new(r"shell\(([^)]+)\)").unwrap();
    let re_save_chat_function = Regex::new(r"save\(([^)]+)\)").unwrap();

    println!("gptrs {} \n", version);
    println!(
        "Type help() for information on commands, print() to print current query, exit() to quit"
    );
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(_) => {}
        Err(_) => {
            exit_with_error("Error: OPENAI_API_KEY environment variable is not set, please set it before continuing \n Create an API Key here https://platform.openai.com/account/api-keys ... \n Exiting ... ");
        }
    }

    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(input) => {
                let history_err = rl.add_history_entry(input.as_str());
                match history_err {
                    //TODO: decide how to handle 
                    _ => {}
                }
                match input.trim() {
                    "exit()" => break,
                    "help()" => {
                        println!("{}", "shell(command) to run a command in the shell and include the output to gpt");
                        println!(
                            "{}",
                            "file(./path/to/file,line,line) to load a file into the query"
                        );
                        println!(
                            "{}",
                            "cat(./path/to/file,line,line) to print the contents of a file"
                        );
                    }
                    "print_chat()" => {
                        println!("Current chat history: ");
                        for message in chat_history.get_all() {
                            let role = message.role.unwrap();
                            println!("User: {:?}", role);
                            if role == "user" {
                                println!("Message: {}", message.content.unwrap().blue());
                            } else {
                                println!("Message: {}", message.content.unwrap().green());
                            }
                        }
                    }
                    "clear_chat()" => {
                        println!("Clearing chat history: ");
                        chat_history.flush();
                    }
                    "chat()" => {
                        chat_history.add(generate_message_from_prompt(&history));

                        let request = chat::ChatCreateCompletionParams {
                            max_tokens: Some(max_tokens),
                            model: Some(models::Models::Gpt35Turbo.name().to_string()),
                            messages: Some(chat_history.get_all()),
                            temperature: Some(0.7),
                        };
                        let output = chat::process_chat_prompt(request).await;
                        match output {
                            Ok(output) => {
                                output.save_messages(&mut chat_history);
                                output.to_cli()
                            }
                            Err(e) => {
                                eprintln!("{}: {:?}", "Error".red(), e)
                            }
                        }
                        history = String::from("");
                    }
                    "complete()" => {
                        let request = completion::CodeCompletionCreateParams {
                            max_tokens: max_tokens,
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
                        } else if let Some(captures) = re_file_function.captures(&input) {
                            let contents_to_use = parse_file(captures, false);
                            history.push_str(&contents_to_use);
                        } else if let Some(captures) = re_shell_function.captures(&input) {
                            parse_terminal_command(captures);
                        } else if let Some(captures) = re_save_chat_function.captures(&input) {
                            println!("TODO: save chat to output file: command: {:?}", captures);
                        } else if let Some(captures) = re_max_tokens_function.captures(&input) {
                            max_tokens = get_max_tokens(captures);
                            println!("Setting max tokens to {:?}", max_tokens);
                        } else {
                            history.push_str(&input);
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
