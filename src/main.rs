mod completion;
use std;
use std::fs;
use text_colorizer::*;
use std::env;
use serde_json::Result as SerdeResult;
use serde_json::{from_str};
use clap::{arg, Command, ArgMatches};

fn cli() -> Command {
    Command::new("gptrs")
    .subcommand( Command::new("completion")
    .args([
        arg!(--prompt <PROMPT> "Prompt to enter in chatgptm if this is included with a file it will be added to the top of the file as a comment"),
        arg!(--file <FILE> "Include a file to get a response from chatgpt, a prompt also needs to be added (see --prompt) to give the API direction"),
        // todo, add these globally
        arg!(--max-tokens <MAX_TOKENS> "Max tokens depends on model, see --model"),
        arg!(--temperature <TEMPERATURE> "Value from 0-1, Lower temperatures give more precise results."),
        arg!(--model <MODEL> "For code completion, use `code-davinci-002` (latest beta, up to 4000 tokens) or `code-cushman-001` (up to 2048 tokens) \n for gpt models use text-davinci-003	"),
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

async fn parse_completion_file(api_key: &str, prompt: &str, path: &str, request_defaults: completion::RequestBase) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let prompt = format!("\\\\ {} \n{}", prompt, contents);
    process_completion_prompt(&prompt, api_key, request_defaults).await
}

async fn process_completion_prompt(prompt: &str, api_key: &str, request_defaults: completion::RequestBase) {
    //TODO: Readd Language
    let result = completion::send_request(&prompt, &api_key, request_defaults).await;
    println!("");
    match result {
        Ok(response) => {
            match parse_completion_response(response) {
                Ok(completion) => {
                    println!("---Prompt---");
                    println!("");
                    println!("{}", &prompt.cyan());
                    println!("");
                    println!("---Response---");
                    println!("");
                    for choice in completion.choices.iter() {
                        let lines = choice.text.lines();
                        for line in lines {
                            if line.trim().is_empty() {
                                continue; // ignore empty or whitespace-only lines
                            }
                            println!("{}", line.green())
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


fn get_request_defaults(matches: ArgMatches) -> completion::RequestBase {
    let accepted_models = vec!["text-davinci-003", "code-davinci-002", "code-cushman-001"];
    // set defaults
    let mut model = "text-davinci-003";
    let mut temperature = 0.7;
    let mut max_tokens = 300;

    let model_arg = matches.get_one::<String>("model").map(|s| s.as_str());
    if let Some(model_arg) = model_arg {
        if !accepted_models.contains(&model_arg) {
            println!("{} not found in accepted list {:?}, so defaulting to {}", model_arg, accepted_models, model);
        };
        if accepted_models.contains(&model_arg) {
            model = model_arg;
        }
    }

    let temperature_arg = matches.get_one::<f32>("temperature").map(|s| s);
    if let Some(temperature_arg) = temperature_arg {
        // todo: add check
        temperature = temperature_arg.clone()
    }

    let max_tokens_arg = matches.get_one::<i32>("max_tokens").map(|s| s);
    if let Some(max_tokens_arg) = max_tokens_arg {
        // todo: add check
        max_tokens = max_tokens_arg.clone()
    }

    completion::RequestBase { model: model.to_string(), temperature: temperature, max_tokens: max_tokens}
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
    let request_defaults = get_request_defaults(matches.clone());

    if let Some(completion_matches) = matches.subcommand_matches("completion") {
        let prompt = completion_matches.get_one::<String>("prompt").map(|s| s.as_str());
        match prompt {
            Some(prompt) => {
                // if there is a file, then inject prompt with file
                let file = completion_matches.get_one::<String>("file").map(|s| s.as_str());
                match file {
                    Some(file) => {
                        parse_completion_file(&api_key.clone().unwrap(), prompt, file, request_defaults).await;
                    },
                    None => {
                        process_completion_prompt(&prompt, &api_key.clone().unwrap(), request_defaults).await;
                    },
                }
            },
            None => {},
        }
       
    }
}

