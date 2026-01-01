use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub enum DirStructureFormat {
    List,
    Tree,
}

pub fn dir_structure(dir: &Path, depth: usize, format: &DirStructureFormat) -> String {
    let mut tree = String::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            match format {
                DirStructureFormat::Tree => {
                    let name = path
                        .file_name()
                        .unwrap_or(OsStr::new("NOT_A_FILE"))
                        .to_string_lossy();
                    tree.push_str(&"  ".repeat(depth));
                    tree.push_str(&format!("{name}\n"));
                }
                DirStructureFormat::List => {
                    let path = path.to_string_lossy().to_string();
                    tree.push_str(&format!("{path}\n"));
                }
            }
            if path.is_dir() {
                tree.push_str(&dir_structure(&path, depth + 1, format));
            }
        }
    }
    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_tree() {
        let path = Path::new(".");
        let tree = dir_structure(path, 0, &DirStructureFormat::Tree);
        println!("Directory tree:\n{tree}");
    }
    #[test]
    fn test_dir_list() {
        let path = Path::new(".");
        let list = dir_structure(path, 0, &DirStructureFormat::List);
        println!("Directory list:\n{list}");
    }
}
