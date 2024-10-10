use serde_yaml;
use serde_yaml::Value;
use std::{collections::HashSet, path::PathBuf};

use super::yaml_file_helper::read_project_config_file_as_string;
use super::{
    chains::DataAvailabilityTypes, yaml_file_helper::get_project_config_file_path, ChainConfig,
    ConfigError, FrameworkConfig, FrameworkType, LayerConfig, MoneaProjectConfig,
};

pub fn verify_project_config(relative_project_path: &str) -> Result<(), ConfigError> {
    let config_contents =
        read_project_config_file_as_string(&PathBuf::from(relative_project_path))?;

    // Parse the YAML as a generic Value to check for unknown fields
    let yaml_value: Value = serde_yaml::from_str(&config_contents)?;

    // Define the known top-level fields
    let known_fields: HashSet<&str> = [
        "project_name",
        "layer1",
        "layer2",
        "layer3",
        "layer4",
        "layer5",
        "layer6",
        "layer7",
        "layer8",
        "layer9",
        "layer10",
        "layer11",
        "layer12",
        "layer13",
        "layer14",
    ]
    .iter()
    .cloned()
    .collect();

    // Check for unknown fields
    if let Value::Mapping(map) = yaml_value {
        for key in map.keys() {
            if let Value::String(field_name) = key {
                if !known_fields.contains(field_name.as_str()) {
                    return Err(ConfigError::UnknownField(field_name.clone()));
                }
            }
        }
    }

    // Parse the config now that we've checked for unknown fields
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
    // Define known fields for LayerConfig
    let known_fields: HashSet<&str> = ["pipeline", "chains"].iter().cloned().collect();

    // Check for unknown fields
    if let Value::Mapping(map) = serde_yaml::to_value(layer_config)? {
        for key in map.keys() {
            if let Value::String(field_name) = key {
                if !known_fields.contains(field_name.as_str()) {
                    return Err(ConfigError::UnknownField(format!(
                        "Unknown field '{}' in layer {}",
                        field_name, layer_number
                    )));
                }
            }
        }
    }

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
    // Define known fields for ChainConfig
    let known_fields: HashSet<&str> = [
        "name",
        "chain_id",
        "block_time",
        "framework_type",
        "framework_config",
        "deploy",
        "settlement",
        "data_availability",
    ]
    .iter()
    .cloned()
    .collect();

    // Check for unknown fields
    if let Value::Mapping(map) = serde_yaml::to_value(chain)? {
        for key in map.keys() {
            if let Value::String(field_name) = key {
                if !known_fields.contains(field_name.as_str()) {
                    return Err(ConfigError::UnknownField(format!(
                        "Unknown field '{}' in chain config for layer {}",
                        field_name, layer_number
                    )));
                }
            }
        }
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
