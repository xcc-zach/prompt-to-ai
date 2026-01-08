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
                todo!();
            }
            ConfigAction::UseModel => {
                todo!();
            }
            ConfigAction::RemoveModel => {
                todo!();
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
