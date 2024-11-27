use serde::Deserialize;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("No solutions found in config")]
    NoSolutions,
}

#[derive(Debug, Deserialize)]
pub struct Solution {
    pub day: u8,
    pub part1: String,
    pub part2: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub solutions: Vec<Solution>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        if config.solutions.is_empty() {
            return Err(ConfigError::NoSolutions);
        }

        Ok(config)
    }
}
