use clap::{Parser, Subcommand};
use prompt_to_ai::{
    Clip, ClipFallback, DirStructureFormat, add_commit, dir_structure, get_commit_prompt,
};
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
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Commit { use_english } => {
            let commit_prompt =
                get_commit_prompt(!use_english).expect("Failed to get commit prompt");
            let tmp_file_handle = commit_prompt
                .clip(ClipFallback::Save)
                .expect("Failed to clip commit prompt");
            println!("Please enter commit message:");
            let mut commit_msg = String::new();
            std::io::stdin()
                .read_line(&mut commit_msg)
                .expect("Failed to read line");
            tmp_file_handle
                .map(|handle| handle.cleanup())
                .transpose()
                .expect("Failed to clean up temporary file");
            println!("Committing with message: {}", commit_msg.trim());
            add_commit(commit_msg.trim().to_owned()).expect("Failed to add commit");
            println!("Committed successfully.");
        }
        Command::Ls => {
            let file_structure = dir_structure(Path::new("."), 0, &DirStructureFormat::List);
            file_structure
                .clip(ClipFallback::Print)
                .expect("Failed to clip file structure");
        }
    }
}
