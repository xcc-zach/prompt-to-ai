use arboard::Clipboard;
use std::fs::{File, remove_file};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub trait Clip {
    fn clip(
        &self,
        fallback: ClipFallback,
    ) -> Result<Option<FileHandle>, Box<dyn std::error::Error>>;
}
pub enum ClipFallback {
    Print,
    Save,
}
pub struct FileHandle(File, PathBuf);
pub fn prompt_to_input(target: &mut String, prompt: &str) -> Result<(), io::Error> {
    println!("{prompt}");
    io::stdout().flush()?;
    io::stdin().read_line(target)?;

    let old = std::mem::take(target);
    target.push_str(old.trim());

    Ok(())
}
fn create_temp_file() -> Result<FileHandle, std::io::Error> {
    let path = Path::new("pai_temp.txt");
    let file = File::create(path)?;
    Ok(FileHandle::new(file, path.to_path_buf()))
}
impl Clip for String {
    fn clip(
        &self,
        fallback: ClipFallback,
    ) -> Result<Option<FileHandle>, Box<dyn std::error::Error>> {
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard.set_text(self.clone())?;
            println!("Result copied to clipboard.");
        } else {
            match fallback {
                ClipFallback::Print => {
                    println!("Clipboard not available, falling back to print:");
                    println!("{}", self);
                }
                ClipFallback::Save => {
                    let mut file_handle = create_temp_file()?;
                    file_handle.0.write_all(self.as_bytes())?;
                    file_handle.0.flush()?;
                    println!("Clipboard not available, falling back to save to file.");
                    return Ok(Some(file_handle));
                }
            }
        }
        Ok(None)
    }
}

impl FileHandle {
    pub fn cleanup(self) -> Result<(), std::io::Error> {
        drop(self.0);
        remove_file(self.1)
    }
    fn new(file: File, path: PathBuf) -> Self {
        FileHandle(file, path)
    }
}
