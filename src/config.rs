use clap::builder::Str;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::{fs, path::PathBuf};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Path error: {0}")]
    PathError(String),
    #[error("Read error: {0}")]
    ReadError(String),
    #[error("Write error: {0}")]
    WriteError(String),
}
pub fn add_model_config(
    tag: String,
    api_key: String,
    model: String,
    base_url: Option<String>,
) -> Result<(), ConfigError> {
    todo!()
}

pub fn use_model_config(tag: String) -> Result<(), ConfigError> {
    todo!()
}

pub fn delete_model_config(tag: String) -> Result<(), ConfigError> {
    todo!()
}

#[derive(Default, Serialize, Deserialize)]
struct Config {
    model_config: ModelConfig,
}
#[derive(Default, Serialize, Deserialize)]
struct ModelConfig {
    current_model_tag: Option<String>,
    items: BTreeMap<String, ModelConfigItem>,
}
#[derive(Default, Serialize, Deserialize)]
struct ModelConfigItem {
    api_key: String,
    model: String,
    base_url: Option<String>,
}
fn config_file_path() -> Result<PathBuf, ConfigError> {
    std::env::current_exe()
        .map_err(|e| ConfigError::PathError(e.to_string()))?
        .parent()
        .ok_or(ConfigError::PathError("Path not valid".to_string()))
        .map(|p| p.join("pai_config.toml"))
}
fn save_config(cfg: &Config) -> Result<(), ConfigError> {
    let path = config_file_path()?;
    let content =
        toml::to_string_pretty(cfg).map_err(|e| ConfigError::WriteError(e.to_string()))?;
    fs::write(path, content).map_err(|e| ConfigError::WriteError(e.to_string()))
}
fn load_config() -> Result<Config, ConfigError> {
    let config_path = config_file_path()?;
    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }
    let content =
        fs::read_to_string(config_path).map_err(|e| ConfigError::ReadError(e.to_string()))?;
    toml::from_str(&content).map_err(|e| ConfigError::ReadError(e.to_string()))
}
