use crate::error::ProviderError;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub labels: Vec<super::common::Label>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ProviderError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ProviderError::Config(format!("Failed to read config file: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| ProviderError::Config(format!("Failed to parse config file: {}", e)))
    }
}
