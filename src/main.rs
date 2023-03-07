mod completion;
use clap::{arg, ArgMatches, Command};
use serde_json::from_str;
use serde_json::Result as SerdeResult;
use std;
use std::error::Error;
use std::fmt;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
struct CompletionError {
    message: String,
}

impl CompletionError {
    fn new(message: &str) -> CompletionError {
        CompletionError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CompletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CompletionError {}

trait Output {
    fn get_output(&self) -> String;
    fn to_file(&self, path: String);
    fn to_cli(&self);
    fn parse(&self, output_path: String);
}

impl Output for completion::CodeCompletion {
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

    fn to_file(&self, path: String) {
        std::fs::write(path, self.get_output()).expect("Unable to write file")
    }

    fn to_cli(&self) {
        println!("{}", self.get_output().green())
    }

    fn parse(&self, output_path: String) {
        if output_path != "" {
            self.to_file(output_path)
        } else {
            self.to_cli()
        }
    }
}

struct RequestDefaults {
    matches: ArgMatches,
}

trait Defaults {
    fn get_prompt(&self) -> String;
    fn get_file_path(&self) -> String;
    fn get_output_path(&self) -> String;
    fn get_model(&self) -> String;
    fn get_temperature(&self) -> f32;
    fn get_max_tokens(&self) -> i32;
    fn get_request_base(&self) -> completion::RequestBase;
}

impl Defaults for RequestDefaults {
    fn get_file_path(&self) -> String {
        let file = self
            .matches
            .get_one::<String>("input")
            .map(|s| s.as_str())
            .unwrap_or("");
        file.to_string()
    }

    fn get_output_path(&self) -> String {
        let output_path = self
            .matches
            .get_one::<String>("output")
            .map(|s| s.as_str())
            .unwrap_or("");
        output_path.to_string()
    }

    fn get_prompt(&self) -> String {
        let prompt = self
            .matches
            .get_one::<String>("prompt")
            .map(|s| s.as_str())
            .unwrap();
        let file_path = self.get_file_path();
        if file_path != "" {
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");
            return format!("\\\\ {} \n{}", prompt, contents).to_string();
        }
        prompt.to_string()
    }

    fn get_model(&self) -> String {
        let accepted_models = vec!["text-davinci-003", "code-davinci-002", "code-cushman-001"];
        let mut model = "text-davinci-003";
        let model_arg = self.matches.get_one::<String>("model").map(|s| s.as_str());
        if let Some(model_arg) = model_arg {
            if !accepted_models.contains(&model_arg) {
                println!(
                    "{} not found in accepted list {:?}, so defaulting to {}",
                    model_arg, accepted_models, model
                );
            };
            if accepted_models.contains(&model_arg) {
                model = model_arg;
            }
        }
        model.to_string()
    }

    fn get_temperature(&self) -> f32 {
        let mut temperature = f32::from(0.7);
        let temperature_arg = self
            .matches
            .get_one::<String>("temperature")
            .map(|s| s.parse::<f32>());
        if let Some(temperature_arg) = temperature_arg {
            temperature = temperature_arg.unwrap();
        }
        temperature
    }

    fn get_max_tokens(&self) -> i32 {
        let mut max_tokens = 300;
        let max_tokens_arg = self
            .matches
            .get_one::<String>("max_tokens")
            .map(|s| s.parse::<i32>());
        if let Some(max_tokens_arg) = max_tokens_arg {
            // todo: add check
            max_tokens = max_tokens_arg.unwrap()
        }
        max_tokens
    }

    fn get_request_base(&self) -> completion::RequestBase {
        completion::RequestBase {
            model: self.get_model(),
            temperature: self.get_temperature(),
            max_tokens: self.get_max_tokens(),
            prompt: self.get_prompt(),
        }
    }
}

fn cli() -> Command {
    Command::new("gptrs")
    .subcommand( Command::new("completion")
    .args([
        arg!(--prompt <PROMPT> "Prompt to enter in chatgptm if this is included with a file it will be added to the top of the file as a comment"),
        arg!(--input <INPUT> "Include a file to get a response from chatgpt, a prompt also needs to be added (see --prompt) to give the API direction"),
        arg!(--output <OUTPUT> "Output file destination"),
        arg!(--max_tokens <MAX_TOKENS> "Max tokens depends on model, see --model"),
        arg!(--temperature <TEMPERATURE> "Value from 0-1, Lower temperatures give more precise results."),
        arg!(--model <MODEL> "For code completion, use `code-davinci-002` (latest beta, up to 4000 tokens) or `code-cushman-001` (up to 2048 tokens) \n for gpt models use text-davinci-003	"),
    ]))
}

fn parse_completion_response(response: String) -> SerdeResult<completion::CodeCompletion> {
    match from_str(&response) {
        Ok(c) => return Ok(c),
        Err(e) => {
            return Err(e);
        }
    };
}

async fn process_completion_prompt(
    request_defaults: completion::RequestBase,
) -> Result<completion::CodeCompletion, CompletionError> {
    //TODO: Readd Language
    let result = completion::send_request(request_defaults).await;
    match result {
        Ok(response) => match parse_completion_response(response) {
            Ok(completion) => {
                return Ok(completion);
            }
            Err(e) => {
                return Err(CompletionError::new(&e.to_string()));
            }
        },
        Err(e) => {
            return Err(CompletionError::new(&e.to_string()));
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    if let Some(completion_matches) = matches.subcommand_matches("completion") {
        let request_defaults = RequestDefaults {
            matches: completion_matches.clone(),
        };
        let output_path = request_defaults.get_output_path();
        let output = process_completion_prompt(request_defaults.get_request_base()).await;
        match output {
            Ok(output) => output.parse(output_path),
            Err(e) => {
                eprintln!("{}: {:?}", "Error".red(), e)
            }
        }
    }
}
