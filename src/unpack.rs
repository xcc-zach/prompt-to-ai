use crate::utils::assets::{EmbeddedFile, list_assets};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

const SKILLS_PREFIX: &str = "skills/";
const CLAUDE_PREFIX: &str = "claude/";

fn list_by_prefix(prefix: &'static str) -> impl Iterator<Item = &'static EmbeddedFile> {
    list_assets().filter(move |f| f.path.starts_with(prefix))
}

fn unpack_skills() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;

    let target_dirs = [
        home_dir.join(".claude").join("skills"),
        home_dir.join(".codex").join("skills"),
    ];

    for target_dir in &target_dirs {
        fs::create_dir_all(target_dir)?;
    }

    for file in list_by_prefix(SKILLS_PREFIX) {
        let relative_path = &file.path[SKILLS_PREFIX.len()..];
        if relative_path.is_empty() {
            continue;
        }

        for target_dir in &target_dirs {
            let dest_path: PathBuf = target_dir.join(relative_path);

            // Remove existing file or directory with the same name
            if dest_path.exists() {
                if dest_path.is_dir() {
                    fs::remove_dir_all(&dest_path)?;
                } else {
                    fs::remove_file(&dest_path)?;
                }
            }

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&dest_path, file.data)?;
        }
    }

    Ok(())
}

fn unpack_claude_settings() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let target_dir = home_dir.join(".claude");

    fs::create_dir_all(&target_dir)?;

    for file in list_by_prefix(CLAUDE_PREFIX) {
        let relative_path = &file.path[CLAUDE_PREFIX.len()..];
        if relative_path.is_empty() {
            continue;
        }

        let dest_path: PathBuf = target_dir.join(relative_path);

        // Remove existing file or directory with the same name
        if dest_path.exists() {
            if dest_path.is_dir() {
                fs::remove_dir_all(&dest_path)?;
            } else {
                fs::remove_file(&dest_path)?;
            }
        }

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&dest_path, file.data)?;
    }

    Ok(())
}

fn unpack_mcp() -> Result<(), Box<dyn Error>> {
    println!("Adding GitHub MCP...");
    print!("Enter your GitHub PAT from https://github.com/settings/personal-access-tokens/new: ");
    io::stdout().flush()?;

    let mut pat = String::new();
    io::stdin().read_line(&mut pat)?;
    let pat = pat.trim();

    if pat.is_empty() {
        println!("Skipping GitHub MCP setup.");
        return Ok(());
    }

    let json_config = format!(
        r#"{{"type":"http","url":"https://api.githubcopilot.com/mcp","headers":{{"Authorization":"Bearer {}"}}}}"#,
        pat
    );

    #[cfg(windows)]
    let status = Command::new("cmd")
        .args(["/c", "claude", "mcp", "add-json", "-s", "user", "github", &json_config])
        .status()?;

    #[cfg(not(windows))]
    let status = Command::new("claude")
        .args(["mcp", "add-json", "-s", "user", "github", &json_config])
        .status()?;

    if !status.success() {
        return Err(format!("claude mcp add-json command failed with status: {}", status).into());
    }

    Ok(())
}

pub fn unpack_all() -> Result<(), Box<dyn Error>> {
    unpack_claude_settings()?;
    unpack_skills()?;
    unpack_mcp()?;
    println!("All assets unpacked successfully!");
    Ok(())
}
