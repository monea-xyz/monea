use serde_yaml;
use std::fs;
use std::path::Path;

use super::{
    chains::DataAvailabilityTypes, ChainConfig, ConfigError, FrameworkConfig, FrameworkType,
    LayerConfig, MoneaProjectConfig,
};

pub fn verify_project_config(relative_path: &str) -> Result<(), ConfigError> {
    let config_path;

    let path = Path::new(relative_path);
    // check for either .yaml or .yml
    let yaml_path = path.join("monea.config.yaml");
    let yml_path = path.join("monea.config.yml");

    if yaml_path.exists() {
        config_path = yaml_path;
    } else if yml_path.exists() {
        config_path = yml_path;
    } else {
        return Err(ConfigError::NotFound);
    }

    let config_contents = fs::read_to_string(config_path)?;
    let config: MoneaProjectConfig = serde_yaml::from_str(&config_contents)?;

    // validate project name
    if config.project_name.is_empty() {
        return Err(ConfigError::ValidationError(
            "Project name is required".to_string(),
        ));
    }
    // validate layer1 and layer2
    if let Some(layer1) = &config.layer1 {
        validate_layer_config(layer1, 1)?;
    }
    if let Some(layer2) = &config.layer2 {
        validate_layer_config(layer2, 2)?;
    }

    // // validate layer3 to layer14
    // for layer in 3..=14 {
    //     if let Some(layer_config) = &config.layer(layer) {
    //         validate_layer_config(layer_config, layer)?;
    //     }
    // }

    Ok(())
}

fn validate_layer_config(layer_config: &LayerConfig, layer_number: u64) -> Result<(), ConfigError> {
    // validate chains
    if layer_config.chains.is_empty() {
        return Err(ConfigError::ValidationError(format!(
            "Layer {} must have at least one chain configuration",
            layer_number
        )));
    }

    for chain in &layer_config.chains {
        validate_chain_config(chain, layer_number)?;
    }

    Ok(())
}

fn validate_chain_config(chain: &ChainConfig, layer_number: u64) -> Result<(), ConfigError> {
    if chain.name.is_empty() {
        return Err(ConfigError::ValidationError(
            "Chain name is required".to_string(),
        ));
    }
    if chain.chain_id == 0 {
        return Err(ConfigError::ValidationError(
            "Chain id must be greater than 0".to_string(),
        ));
    }

    // validate framework type and config if present
    if let Some(framework_type) = &chain.framework_type {
        if let Some(framework_config) = &chain.framework_config {
            match (framework_type, framework_config) {
                (FrameworkType::OpStack, FrameworkConfig::OpStack(op_config)) => {
                    if op_config.op_config_file_path.is_empty() {
                        return Err(ConfigError::ValidationError(
                            "OpStack config file path is required when OpStack config is provided"
                                .to_string(),
                        ));
                    }
                }
                (FrameworkType::Custom, FrameworkConfig::Custom(custom_config)) => {
                    if custom_config.custom_config_file_path.is_empty() {
                        return Err(ConfigError::ValidationError(
                            "Custom config file path is required when Custom config is provided"
                                .to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(ConfigError::ValidationError(
                        "Framework type and config mismatch".to_string(),
                    ));
                }
            }
        }
    }

    // validate settlement config if present
    if let Some(settlement) = &chain.settlement {
        if settlement.network_id == 0 {
            return Err(ConfigError::ValidationError(
                "Settlement network id must be greater than 0".to_string(),
            ));
        }
    }

    // validate data availability config if present
    if let Some(data_availability) = &chain.data_availability {
        match data_availability.da_type {
            DataAvailabilityTypes::Blobs => {
                // No additional validation needed for Blobs type
                // as there's no provider field in the current schema
            }
            DataAvailabilityTypes::Calldata => {
                // No additional validation needed for Calldata type
            }
        }
    }

    Ok(())
}
