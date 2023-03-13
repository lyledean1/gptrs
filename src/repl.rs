use crate::chat;
use crate::chat::History;
use crate::chat::Message;
use crate::chat::MessageHistory;
use crate::completion;
use crate::models;
use crate::models::{get_model, Models};
use crate::output::Output;
use regex::{Captures, Regex};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use spinoff::Streams;
use spinoff::{spinners, Color, Spinner};
use std::env;
use std::fs;
use text_colorizer::*;

//TODO: make this into an enum and loop through
fn print_quickstart() {
    println!("{}", "Quickstart:".bold());
    println!(
        "{}{}",
        "\"help()\"".italic(),
        " for information on commands"
    );
    println!("{}{}", "\"print()\"".italic(), " to print current query");
    println!(
        "{}{}",
        "\"chat()\"".italic(),
        " to send the query to the chat API"
    );
    println!(
        "{}{}",
        "\"completion()\"".italic(),
        " to send query to completion API"
    );
    println!("{}{}", "\"exit()\"".italic(), " to exit the terminal");
}

//TODO: make this into an enum and loop through
fn print_help() {
    println!("{}", "Generic Commands".bold());
    println!(
        "{}{}",
        "\"print()\"".blue(),
        " to print the current terminal query "
    );
    println!(
        "{}{}",
        "\"empty()\"".blue(),
        " to clear the current terminal query and start again"
    );
    println!(
        "{}{}",
        "\"cat(\"./path/to/file\",line,line)\"".blue()," to load a file into the query i.e \"file(\"./main.rs\",1,12)\" - the line numbers are optional"
    );
    println!(
        "{}{}",
        "\"file(\"./path/to/file\",line,line)\"".blue()," to load a file into the query i.e \"file(\"./main.rs\",1,12)\" - the line numbers are optional"
    );
    println!("");
    println!("{}", "Completion API Commands".bold());
    println!(
        "{}{}{}",
        "\"complete()\"".yellow()," to send the current terminal query to OpenAI Completion API https://platform.openai.com/docs/guides/code",
        " * note: you must set model to a code model for complete to work i.e model(\"code-cushman-001\")"
    );
    println!("");
    println!("{}", "Chat API Commands".bold());
    println!(
        "{}{}",
        "\"chat()\"".green()," to send the current terminal query to OpenAI Chat API https://platform.openai.com/docs/guides/chat"
    );
    println!(
        "{}{}",
        "\"log()\"".green()," to see the current chat history, each subsequent chat request will include this chat history for context"
    );
    println!(
        "{}{}",
        "\"clear()\"".green(),
        " to clear the current chat history and start a new chat from fresh"
    );
    println!(
        "{}{}",
        "\"export(\"./path/to/file.json\")\"".green(),
        " to export the current chat history to a json file"
    );
    println!("");
    println!("{}", "API Configuration".bold());
    println!("{}", "Note: currently for both chat and completion you can only change model,max_tokens and temperature. This will be expanded.");
    println!("");
    println!(
        "{}{}",
        "\"models()\"".cyan(), " to see a list of models, also see https://platform.openai.com/docs/models incase the model you require is not listed, you can still supply it as a string i.e \"model-name-001\""
    );
    println!(
        "{}{}",
        "\"model(\"model\")\"".cyan(),
        " to set a model i.e model(\"code-cushman-001\")"
    );
    println!(
        "{}{}",
        "\"max_tokens(3000)\"".cyan(),
        " to set max_tokens, default is 300"
    );
    println!(
        "{}{}",
        "\"temperature(0.5)\"".cyan(),
        " to set temperature, default is 0.7, allowed values are 0 to 1"
    );
    println!("");
}

fn print_all_models() {
    for model in Models::all() {
        println!(
            "Model '{}' has max tokens of {}",
            model.name(),
            model.max_tokens()
        );
        println!("Description: {}", model.description());
        println!("");
    }
    println!(
        "{}",
        "To set a model, run model(\"model\") i.e model(\"code-cushman-001\")".yellow()
    );
    println!("");
}

fn string_to_vec(s: &str) -> Vec<String> {
    vec![String::from(s)]
}

fn get_max_tokens(captures: Captures) -> i32 {
    let args: Vec<&str> = captures[1].split(",").collect();
    args[0].parse::<i32>().unwrap()
}

fn check_model_input(captures: Captures) -> Models {
    let args: Vec<&str> = captures[1].split(",").collect();
    let model = args[0].trim_matches('"').parse::<String>().unwrap();
    get_model(&model)
}

fn check_temperature_input(captures: Captures) -> f64 {
    let args: Vec<&str> = captures[1].split(",").collect();
    let temperature = args[0].parse::<f64>().unwrap();
    let default = 0.7;
    if temperature > 1.0 || temperature < 0.0 {
        return default;
    }
    temperature
}

fn export_chat_to_ouput(captures: Captures, chat_history: Vec<Message>) {
    let args: Vec<&str> = captures[1].split(",").collect();
    let file_path = args[0].trim_matches('"').parse::<String>().unwrap();
    let json = serde_json::to_string(&chat_history);
    match json {
        Ok(json) => {
            println!("Saving chat history to {}", file_path);
            _ = fs::write(file_path, json);
        }
        Err(_) => {}
    }
}

fn parse_file(captures: Captures, print: bool) -> String {
    let args: Vec<&str> = captures[1].split(",").collect();
    if args.len() > 3 {
        eprintln!("Error {}", "Only maximum of 3 args are allowed".red());
    }
    //HACK: to handle strings inside quotes (need to convert this into a actual parser with grammar)
    let file_path = args[0].trim_matches('"');
    let contents = std::fs::read_to_string(file_path);
    let mut contents_to_use = String::from("");
    match contents {
        Ok(lines) => {
            for (index, line) in lines.lines().enumerate() {
                if args.len() > 1 {
                    let start = args[1].trim().parse::<usize>().unwrap();
                    if index < (start - 1) {
                        continue;
                    }
                }
                if args.len() > 2 {
                    let end = args[2].trim().parse::<usize>().unwrap();
                    if index > (end - 1) {
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
    let mut temperature = 0.7;
    let mut model = models::Models::Gpt35Turbo;
    let version: &str = env!("CARGO_PKG_VERSION");

    //TODO: let this take strings in the form function("string", 1, 2) where 1 and 2 are optional arguments
    //TODO: convert this into a grammar and parse vs using Regex
    let re_cat_function = Regex::new(r"^cat\(([^)]+)\)").unwrap();
    let re_max_tokens_function = Regex::new(r"^max_tokens\(([^)]+)\)").unwrap();
    let re_file_function = Regex::new(r"^file\(([^)]+)\)").unwrap();
    let re_set_model_function = Regex::new(r"^model\(([^)]+)\)").unwrap();
    let re_set_temperature_function = Regex::new(r"^temperature\(([^)]+)\)").unwrap();
    let re_export_chat_function = Regex::new(r"^export\(([^)]+)\)").unwrap();

    println!("{} version: {}", "gptshell".bold(), version.italic());
    println!("");
    print_quickstart();
    let api_key = env::var("OPENAI_API_KEY");
    match api_key.clone() {
        Ok(_) => {}
        Err(_) => {
            exit_with_error("Error: OPENAI_API_KEY environment variable is not set, please set it before continuing \n Create an API Key here https://platform.openai.com/account/api-keys ... \n Exiting ... ");
        }
    }

    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("history.txt").is_err() {
        println!("");
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
                    "help()" => print_help(),
                    "print()" => {
                        println!("{}", history);
                    }
                    "empty()" => {
                        history = String::from("");
                    }
                    "clear()" => {
                        chat_history.flush();
                        history = String::from("");
                    }
                    "log()" => {
                        let all_chats = chat_history.get_all();
                        if all_chats.len() == 0 {
                            println!("There is no current chat history to display");
                            continue;
                        }
                        println!("Current log of chat history: ");
                        for message in chat_history.get_all() {
                            let role = message.role.unwrap();
                            println!("User: {:?}", role);
                            if role == "user" {
                                println!("Message: {}", message.content.unwrap().blue());
                            } else {
                                println!("Message: {}", message.content.unwrap().green());
                            }
                        }
                        println!("Current query: ")
                    }
                    "chat()" => {
                        chat_history.add(generate_message_from_prompt(&history));

                        let spinner = Spinner::new_with_stream(
                            spinners::Dots,
                            "",
                            Color::Yellow,
                            Streams::Stderr,
                        );
                        let request = chat::ChatCreateCompletionParams {
                            max_tokens: Some(max_tokens),
                            model: Some(model.name().to_string()),
                            messages: Some(chat_history.get_all()),
                            temperature: Some(temperature),
                        };
                        let output = chat::process_chat_prompt(request).await;
                        match output {
                            Ok(output) => {
                                spinner.stop();
                                output.save_messages(&mut chat_history);
                                output.to_cli();
                            }
                            Err(e) => {
                                spinner.stop();
                                eprintln!("{}: {:?}", "Error".red(), e)
                            }
                        }
                        history = String::from("");
                    }
                    "complete()" => {
                        let spinner = Spinner::new_with_stream(
                            spinners::Dots,
                            "",
                            Color::Yellow,
                            Streams::Stderr,
                        );
                        let request = completion::CodeCompletionCreateParams {
                            max_tokens: max_tokens,
                            model: model.name().to_string(),
                            prompt: string_to_vec(&history),
                            temperature: temperature,
                        };
                        let output = completion::process_completion_prompt(request).await;
                        match output {
                            Ok(output) => {
                                spinner.stop();
                                output.to_cli()
                            }
                            Err(e) => {
                                spinner.stop();
                                eprintln!("Note: you must set model to a code model for complete to work i.e model(\"code-cushman-001\")");
                                eprintln!("{}: {:?}", "Completion Error".red(), e)
                            }
                        }
                        history = String::from("");
                    }
                    "models()" => {
                        print_all_models();
                    }
                    _ => {
                        // Parse Regex
                        if let Some(captures) = re_cat_function.captures(&input) {
                            parse_file(captures, true);
                        } else if let Some(captures) = re_file_function.captures(&input) {
                            let contents_to_use = parse_file(captures, false);
                            history.push_str(&contents_to_use);
                        } else if let Some(captures) = re_export_chat_function.captures(&input) {
                            export_chat_to_ouput(captures, chat_history.get_all());
                        } else if let Some(captures) = re_max_tokens_function.captures(&input) {
                            max_tokens = get_max_tokens(captures);
                            println!("Setting max tokens to {:?}", max_tokens);
                        } else if let Some(captures) = re_set_model_function.captures(&input) {
                            model = check_model_input(captures);
                            println!("Setting model to {:?}", model.name());
                        } else if let Some(captures) = re_set_temperature_function.captures(&input)
                        {
                            temperature = check_temperature_input(captures);
                            println!("Setting temperature to {:?}", temperature);
                        } else {
                            history.push_str(&input);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Did you want to exit? Type exit()");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
