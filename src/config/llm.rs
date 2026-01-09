use super::utils::ConfigError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::{fs, path::PathBuf};
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ModelConfigItem {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}
pub fn add_model_config(
    tag: String,
    api_key: String,
    model: String,
    base_url: String,
) -> Result<(), ConfigError> {
    with_model_config(|mc| {
        let model_items = &mc.items;
        if model_items.contains_key(&tag) {
            return Err(ConfigError::ConfigWriteError(format!(
                "model already exists: {tag}"
            )));
        }
        mc.current_model_tag.get_or_insert_with(|| tag.clone());
        mc.items.insert(
            tag,
            ModelConfigItem {
                api_key,
                model,
                base_url,
            },
        );
        Ok(())
    })
}

pub fn use_model_config(tag: String) -> Result<(), ConfigError> {
    with_model_config(|mc| {
        require_model_tag(mc, tag, |mc, tag| {
            mc.current_model_tag = Some(tag);
            Ok(())
        })
    })
}

pub fn delete_model_config(tag: String) -> Result<(), ConfigError> {
    with_model_config(|mc| {
        require_model_tag(mc, tag, |mc, tag| {
            mc.items.remove(&tag);
            Ok(())
        })
    })
}

pub fn get_model_configs() -> Result<ModelConfigMap, ConfigError> {
    let model_config = load_config()?.model_config;
    Ok(model_config.items)
}

pub fn current_model_config() -> Result<ModelConfigItem, ConfigError> {
    let model_config = load_config()?.model_config;
    let model_tag = &model_config
        .current_model_tag
        .ok_or(ConfigError::ConfigReadError(
            "no model configured".to_string(),
        ))?;
    model_config
        .items
        .get(model_tag)
        .ok_or(ConfigError::ConfigReadError(format!(
            "model {model_tag} not registered"
        )))
        .cloned()
}

#[derive(Default, Serialize, Deserialize)]
struct Config {
    model_config: ModelConfig,
}
type ModelConfigMap = BTreeMap<String, ModelConfigItem>;
#[derive(Default, Serialize, Deserialize)]
struct ModelConfig {
    current_model_tag: Option<String>,
    items: ModelConfigMap,
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
        toml::to_string_pretty(cfg).map_err(|e| ConfigError::FileWriteError(e.to_string()))?;
    fs::write(path, content).map_err(|e| ConfigError::FileWriteError(e.to_string()))
}
fn load_config() -> Result<Config, ConfigError> {
    let config_path = config_file_path()?;
    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }
    let content =
        fs::read_to_string(config_path).map_err(|e| ConfigError::FileReadError(e.to_string()))?;
    toml::from_str(&content).map_err(|e| ConfigError::FileReadError(e.to_string()))
}

fn with_model_config<F, T>(f: F) -> Result<T, ConfigError>
where
    F: FnOnce(&mut ModelConfig) -> Result<T, ConfigError>,
{
    let mut cfg = load_config()?;
    let result = f(&mut cfg.model_config)?;
    save_config(&cfg)?;
    Ok(result)
}

fn require_model_tag<F, T>(
    model_config: &mut ModelConfig,
    tag: String,
    f: F,
) -> Result<T, ConfigError>
where
    F: FnOnce(&mut ModelConfig, String) -> Result<T, ConfigError>,
{
    if !model_config.items.contains_key(&tag) {
        return Err(ConfigError::ConfigReadError(format!("no model tag {tag}")));
    }
    f(model_config, tag)
}
