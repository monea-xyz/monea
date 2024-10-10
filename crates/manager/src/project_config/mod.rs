pub mod chains;
pub mod frameworks;
pub mod pipeline;
pub mod verifier;
pub mod yaml_file_helper;
use chains::{ChainConfig, DataAvailabilityConfig, DataAvailabilityTypes, SettlementConfig};
use frameworks::{FrameworkConfig, FrameworkType};
use pipeline::PipelineConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io::Write, path::Path};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerConfig {
    pub pipeline: Option<PipelineConfig>,
    pub chains: Vec<ChainConfig>,
}

// We arbitrarily support up to 14 layers
#[derive(Debug, Serialize, Deserialize)]
pub struct MoneaProjectConfig {
    pub project_name: String,
    pub layer1: Option<LayerConfig>,
    pub layer2: Option<LayerConfig>,
    pub layer3: Option<LayerConfig>,
    pub layer4: Option<LayerConfig>,
    pub layer5: Option<LayerConfig>,
    pub layer6: Option<LayerConfig>,
    pub layer7: Option<LayerConfig>,
    pub layer8: Option<LayerConfig>,
    pub layer9: Option<LayerConfig>,
    pub layer10: Option<LayerConfig>,
    pub layer11: Option<LayerConfig>,
    pub layer12: Option<LayerConfig>,
    pub layer13: Option<LayerConfig>,
    pub layer14: Option<LayerConfig>,
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
    #[error("Unknown field in config: {0}")]
    UnknownField(String),
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

            layer1: Some(LayerConfig {
                pipeline: None,
                chains: vec![ChainConfig {
                    name: "ethereum-l1".to_string(),
                    chain_id: 1,
                    block_time: Some(12),
                    framework_type: None,
                    framework_config: None,
                    deploy: None,
                    settlement: None,
                    data_availability: None,
                }],
            }),
            layer2: Some(LayerConfig {
                pipeline: Some(PipelineConfig {
                    pre: None,
                    post: None,
                }),
                chains: vec![ChainConfig {
                    name: "monea-l2".to_string(),
                    chain_id: 2151908,
                    deploy: Some(true),
                    block_time: Some(2),
                    framework_type: Some(FrameworkType::OpStack),
                    framework_config: None,
                    settlement: Some(SettlementConfig {
                        network_id: 2151908,
                    }),
                    data_availability: Some(DataAvailabilityConfig {
                        da_type: DataAvailabilityTypes::Blobs,
                    }),
                }],
            }),
            layer3: None,
            layer4: None,
            layer5: None,
            layer6: None,
            layer7: None,
            layer8: None,
            layer9: None,
            layer10: None,
            layer11: None,
            layer12: None,
            layer13: None,
            layer14: None,
        };

        let config_content = serde_yaml::to_string(&config)?;
        let mut file = fs::File::create(config_file_path)?;
        file.write_all(config_content.as_bytes())?;

        Ok(config)
    }

    pub fn verify(project_path: &Path) -> Result<(), ConfigError> {
        verifier::verify_project_config(project_path.to_str().unwrap())
    }

    pub fn from_file(project_path: &PathBuf) -> Result<Self, ConfigError> {
        let project_config_path = yaml_file_helper::get_project_config_file_path(project_path)?;
        yaml_file_helper::read_project_config_file_as_struct(&project_config_path)
    }

    pub fn save(&self, project_config_path: &Path) -> Result<(), ConfigError> {
        let config_content = serde_yaml::to_string(self).map_err(|e| ConfigError::ParseError(e))?;

        fs::write(project_config_path, config_content).map_err(ConfigError::ReadError)?;

        Ok(())
    }
}
