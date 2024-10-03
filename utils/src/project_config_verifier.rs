use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct MoneaProjectConfig {
    project_name: String,
    chains: Vec<ChainConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChainConfig {
    name: String,
    rpc_url: String,
    chain_id: u64,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found")]
    NotFound,
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Invalid config: {0}")]
    ValidationError(String),
}

pub fn verify_config(relative_path: &str) -> Result<(), ConfigError> {
    let config_path = find_config_file(relative_path)?;
    let config_contents = fs::read_to_string(config_path)?;
    let config: MoneaProjectConfig = serde_yaml::from_str(&config_contents)?;

    // perform additional validations
    validate_config(&config)?;

    Ok(())
}

fn find_config_file(relative_path: &str) -> Result<PathBuf, ConfigError> {
    let path = Path::new(relative_path);
    let yaml_path = path.join("monea.config.yaml");
    let yml_path = path.join("monea.config.yml");

    if yaml_path.exists() {
        Ok(yaml_path)
    } else if yml_path.exists() {
        Ok(yml_path)
    } else {
        Err(ConfigError::NotFound)
    }
}

fn validate_config(config: &MoneaProjectConfig) -> Result<(), ConfigError> {
    if config.project_name.is_empty() {
        return Err(ConfigError::ValidationError(
            "Project name is required".to_string(),
        ));
    }

    if config.chains.is_empty() {
        return Err(ConfigError::ValidationError(
            "At least one chain configuration is required".to_string(),
        ));
    }

    for chain in &config.chains {
        if chain.name.is_empty() {
            return Err(ConfigError::ValidationError(
                "Chain name is required".to_string(),
            ));
        }
        if chain.rpc_url.is_empty() {
            return Err(ConfigError::ValidationError(
                "Chain RPC URL is required".to_string(),
            ));
        }
        if chain.chain_id == 0 {
            return Err(ConfigError::ValidationError(
                "Chain ID must be greater than 0".to_string(),
            ));
        }
    }

    Ok(())
}
