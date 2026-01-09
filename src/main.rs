use clap::{Parser, Subcommand};
use prompt_to_ai::*;
use std::path::Path;

#[derive(Parser)]
#[command(name = "pai")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Commit {
        #[arg(short = 'e', long = "english", default_value_t = false)]
        use_english: bool,
    },
    Ls,
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    AddModel,
    UseModel,
    RemoveModel,
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Config { action } => match action {
            ConfigAction::AddModel => {
                let mut tag = String::new();
                let mut model = String::new();
                let mut api_key = String::new();
                let mut base_url = String::new();
                prompt_to_input(&mut tag, "Enter tag:")
                    .unwrap_or_else(|e| eprintln!("Error occurred while entering tag: {e}"));
                prompt_to_input(&mut model, "Enter model name:")
                    .unwrap_or_else(|e| eprintln!("Error occurred while entering model name: {e}"));
                prompt_to_input(&mut api_key, "Enter api key:")
                    .unwrap_or_else(|e| eprintln!("Error occurred while entering api key: {e}"));
                prompt_to_input(&mut base_url, "Enter base url (if any)")
                    .unwrap_or_else(|e| eprintln!("Error occurred while entering base url: {e}"));
                config::add_model_config(tag, api_key, model, base_url)
                    .unwrap_or_else(|e| eprintln!("Error occurred while adding model: {e}"));
                println!("Model config added!");
            }
            ConfigAction::UseModel => {
                let mut tag = String::new();
                prompt_to_input(&mut tag, "Enter tag to switch to:")
                    .unwrap_or_else(|e| eprintln!("Error occurred while entering tag: {e}"));
                config::use_model_config(tag)
                    .unwrap_or_else(|e| eprintln!("Error occurred while switching model: {e}"));
                println!("Model switched!");
            }
            ConfigAction::RemoveModel => {
                let mut tag = String::new();
                prompt_to_input(&mut tag, "Enter tag to remove:")
                    .unwrap_or_else(|e| eprintln!("Error occurred while entering tag: {e}"));
                config::delete_model_config(tag)
                    .unwrap_or_else(|e| eprintln!("Error occurred while removing model: {e}"));
                println!("Model config removed!");
            }
        },
        Command::Commit { use_english } => {
            let commit_prompt =
                commit::get_commit_prompt(!use_english).expect("Failed to get commit prompt");
            let tmp_file_handle = commit_prompt
                .clip(ClipFallback::Save)
                .expect("Failed to clip commit prompt");
            let mut commit_msg = String::new();
            prompt_to_input(&mut commit_msg, "Please enter commit message:").expect("IO failed");
            tmp_file_handle
                .map(|handle| handle.cleanup())
                .transpose()
                .expect("Failed to clean up temporary file");
            println!("Committing with message: {}", commit_msg.trim());
            commit::add_commit(commit_msg.trim().to_owned()).expect("Failed to add commit");
            println!("Committed successfully.");
        }
        Command::Ls => {
            let file_structure =
                ls::dir_structure(Path::new("."), 0, &ls::DirStructureFormat::List);
            file_structure
                .clip(ClipFallback::Print)
                .expect("Failed to clip file structure");
        }
    }
}
