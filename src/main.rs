use clap::{Parser, Subcommand};
use prompt_to_ai::*;
use std::path::Path;

#[derive(Parser)]
#[command(name = "pai", about = "A CLI tool for LLM-powered developer workflows")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Generate and create a git commit using LLM")]
    Commit {
        #[arg(short = 'e', long = "english", default_value_t = false, help = "Force English language for commit message")]
        use_english: bool,
        #[arg(long = "auto", default_value_t = false, help = "Automatically generate commit message without manual input")]
        auto: bool,
    },
    #[command(about = "List directory contents and copy to clipboard")]
    Ls,
    #[command(about = "Manage LLM model configurations")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    #[command(about = "Extract bundled assets to set up Claude Code and Codex environments")]
    Unpack,
}

#[derive(Subcommand)]
enum ConfigAction {
    #[command(about = "Add a new LLM model configuration")]
    AddModel,
    #[command(about = "Switch to a different model configuration")]
    UseModel,
    #[command(about = "Remove a model configuration")]
    RemoveModel,
    #[command(about = "List all saved model configurations")]
    ListModels,
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
                    .map_err(|e| eprintln!("Error occurred while entering tag: {e}"))
                    .unwrap();
                prompt_to_input(&mut model, "Enter model name:")
                    .map_err(|e| eprintln!("Error occurred while entering model name: {e}"))
                    .unwrap();
                prompt_to_input(&mut api_key, "Enter api key:")
                    .map_err(|e| eprintln!("Error occurred while entering api key: {e}"))
                    .unwrap();
                prompt_to_input(&mut base_url, "Enter base url:")
                    .map_err(|e| eprintln!("Error occurred while entering base url: {e}"))
                    .unwrap();
                config::llm::add_model_config(tag, api_key, model, base_url)
                    .map_err(|e| eprintln!("Error occurred while adding model: {e}"))
                    .unwrap();
                println!("Model config added!");
            }
            ConfigAction::UseModel => {
                let mut tag = String::new();
                prompt_to_input(&mut tag, "Enter tag to switch to:")
                    .map_err(|e| eprintln!("Error occurred while entering tag: {e}"))
                    .unwrap();
                config::llm::use_model_config(tag)
                    .map_err(|e| eprintln!("Error occurred while switching model: {e}"))
                    .unwrap();
                println!("Model switched!");
            }
            ConfigAction::RemoveModel => {
                let mut tag = String::new();
                prompt_to_input(&mut tag, "Enter tag to remove:")
                    .map_err(|e| eprintln!("Error occurred while entering tag: {e}"))
                    .unwrap();
                config::llm::delete_model_config(tag)
                    .map_err(|e| eprintln!("Error occurred while removing model: {e}"))
                    .unwrap();
                println!("Model config removed!");
            }
            ConfigAction::ListModels => {
                println!("Model configs:");
                let model_configs = config::llm::get_model_configs()
                    .map_err(|e| eprintln!("Error occurred while reading model configs: {e}"))
                    .unwrap();
                for (key, item) in &model_configs {
                    println!("######################################");
                    println!("{key}");
                    println!("{} {}", item.model, item.base_url);
                    println!("{}", item.api_key);
                }
            }
        },
        Command::Commit { use_english, auto } => {
            let commit_prompt = commit::get_commit_prompt(!use_english)
                .map_err(|e| eprintln!("Failed to get commit prompt: {e}"))
                .unwrap();
            let mut commit_msg = String::new();
            if auto {
                println!("Auto generating commit msg with LLM...");
                commit_msg.push_str(
                    &commit::get_commit_message(commit_prompt)
                        .map_err(|e| eprintln!("Error occurred while generating commit msg: {e}"))
                        .unwrap(),
                );
            } else {
                let tmp_file_handle = commit_prompt
                    .clip(ClipFallback::Save)
                    .map_err(|e| eprintln!("Failed to clip commit prompt: {e}"))
                    .unwrap();
                prompt_to_input(&mut commit_msg, "Please enter commit message:")
                    .map_err(|e| eprintln!("IO failed: {e}"))
                    .unwrap();
                tmp_file_handle
                    .map(|handle| handle.cleanup())
                    .transpose()
                    .map_err(|e| eprintln!("Failed to clean up temporary file: {e}"))
                    .unwrap();
            }
            println!("Committing with message: {}", commit_msg.trim());
            commit::add_commit(commit_msg.trim().to_owned())
                .map_err(|e| eprintln!("Failed to add commit: {e}"))
                .unwrap();
            println!("Committed successfully.");
        }
        Command::Ls => {
            let file_structure =
                ls::dir_structure(Path::new("."), 0, &ls::DirStructureFormat::List);
            file_structure
                .clip(ClipFallback::Print)
                .map_err(|e| eprintln!("Failed to clip file structure: {e}"))
                .unwrap();
        }
        Command::Unpack => {
            unpack::unpack_all()
                .map_err(|e| eprintln!("Failed to unpack assets: {e}"))
                .unwrap();
        }
    }
}
