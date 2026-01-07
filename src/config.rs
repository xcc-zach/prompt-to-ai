use thiserror::Error;
#[derive(Error, Debug)]
pub enum ConfigError {}
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
