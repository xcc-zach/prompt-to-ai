use crate::utils::assets::list_skills;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn unpack_skills() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;

    let target_dirs = [
        home_dir.join(".claude").join("skills"),
        home_dir.join(".codex").join("skills"),
    ];

    for target_dir in &target_dirs {
        fs::create_dir_all(target_dir)?;
    }

    let skills_prefix = "skills/";
    for file in list_skills() {
        let relative_path = &file.path[skills_prefix.len()..];
        if relative_path.is_empty() {
            continue;
        }

        for target_dir in &target_dirs {
            let dest_path: PathBuf = target_dir.join(relative_path);

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&dest_path, file.data)?;
            println!("Unpacked: {}", dest_path.display());
        }
    }

    println!("Skills unpacked successfully!");
    Ok(())
}
