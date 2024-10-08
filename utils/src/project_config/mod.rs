pub mod frameworks;
pub mod verifier;
use std::{fs, io::Write, path::Path};

use frameworks::FrameworkConfig;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum FrameworkType {
    OpStack,
    ArbitrumNitro,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainConfig {
    pub name: String,

    pub chain_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<FrameworkType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_config: Option<FrameworkConfig>,

    pub layer_number: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_network: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_availability: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // genesis_file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_layer_one: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_layer_one: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_layer_two: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_layer_two: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_layer_three: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_layer_three: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneaProjectConfig {
    pub project_name: String,
    pub chains: Vec<ChainConfig>,
    pub pipeline: PipelineConfig,
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

impl MoneaProjectConfig {
    pub fn new(
        project_path: &Path,
        name: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_file_path = project_path.join("monea.config.yaml");
        if config_file_path.exists() {
            return Err(format!(
                "monea.config.yaml already exists at {}",
                config_file_path.display()
            )
            .into());
        }

        let project_name = name.unwrap_or_else(|| "Monea Appchain".to_string());

        let config = Self {
            project_name: project_name.clone(),
            chains: vec![
                ChainConfig {
                    name: "ethereum-l1".to_string(),
                    chain_id: 1,
                    framework: None,
                    layer_number: 1,
                    block_time: Some(12),
                    framework_config: None,
                    settlement_network: None,
                    data_availability: None,
                },
                ChainConfig {
                    name: "monea-l2".to_string(),
                    chain_id: 2151908,
                    framework: Some(FrameworkType::OpStack),
                    layer_number: 2,
                    block_time: Some(2),
                    framework_config: None,
                    settlement_network: Some("ethereum-l1".to_string()),
                    data_availability: Some("ethereum-l1".to_string()),
                },
            ],
            pipeline: PipelineConfig {
                pre_layer_one: None,
                post_layer_one: None,
                pre_layer_two: None,
                post_layer_two: None,
                pre_layer_three: None,
                post_layer_three: None,
            },
        };

        let config_content = serde_yaml::to_string(&config)?;
        let mut file = fs::File::create(config_file_path)?;
        file.write_all(config_content.as_bytes())?;

        Ok(config)
    }

    pub fn verify(project_config_path: &Path) -> Result<(), ConfigError> {
        verifier::verify_config(project_config_path.to_str().unwrap())
    }
}
