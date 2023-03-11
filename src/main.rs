mod chat;
mod cli;
mod completion;
mod err;
mod http_client;
mod models;
mod output;
mod repl;
use crate::cli::Defaults;
use crate::output::Output;
use text_colorizer::*;

#[tokio::main]
async fn main() {
    let matches = cli::cli().get_matches();
    if let Some(completion_matches) = matches.subcommand_matches("completion") {
        let request_defaults = cli::RequestDefaults {
            matches: completion_matches.clone(),
        };
        let output_path = request_defaults.get_output_path();
        let output =
            completion::process_completion_prompt(request_defaults.get_request_base()).await;
        match output {
            Ok(output) => output.parse(output_path),
            Err(e) => {
                eprintln!("{}: {:?}", "Error".red(), e)
            }
        }
    } else {
        repl::run_repl().await;
    }
}
