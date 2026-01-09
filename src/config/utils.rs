use thiserror::Error;
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Path error: {0}")]
    PathError(String),
    #[error("File read error: {0}")]
    FileReadError(String),
    #[error("File write error: {0}")]
    FileWriteError(String),
    #[error("Config write error: {0}")]
    ConfigWriteError(String),
    #[error("Config read error: {0}")]
    ConfigReadError(String),
}
