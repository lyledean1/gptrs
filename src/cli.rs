use crate::completion::CodeCompletionCreateParams;
use clap::{arg, ArgMatches, Command};
use std;
use std::fs;

pub struct RequestDefaults {
    pub matches: ArgMatches,
}

pub trait Defaults {
    fn get_prompt(&self) -> Vec<String>;
    fn get_file_path(&self) -> String;
    fn get_output_path(&self) -> String;
    fn get_model(&self) -> String;
    fn get_temperature(&self) -> f64;
    fn get_max_tokens(&self) -> i32;
    fn get_request_base(&self) -> CodeCompletionCreateParams;
}

fn string_to_vec(s: &str) -> Vec<String> {
    vec![String::from(s)]
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

    fn get_prompt(&self) -> Vec<String> {
        let prompt = self
            .matches
            .get_one::<String>("prompt")
            .map(|s| s.as_str())
            .unwrap();
        let file_path = self.get_file_path();
        if file_path != "" {
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");
            return string_to_vec(&format!("\\\\ {} \n{}", prompt, contents).to_string());
        }
        string_to_vec(&prompt.to_string())
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

    fn get_temperature(&self) -> f64 {
        let mut temperature = f64::from(0.7);
        let temperature_arg = self
            .matches
            .get_one::<String>("temperature")
            .map(|s| s.parse::<f64>());
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

    fn get_request_base(&self) -> CodeCompletionCreateParams {
        CodeCompletionCreateParams {
            model: self.get_model(),
            temperature: self.get_temperature(),
            max_tokens: self.get_max_tokens(),
            prompt: self.get_prompt(),
        }
    }
}

pub fn cli() -> Command {
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
