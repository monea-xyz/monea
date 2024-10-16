use monea_utils::path_helper::get_dot_monea_global_path;
use std::{collections::HashMap, error::Error, fs};

use super::{Chain, Config, Enclave, Service};

const DEFAULT_CHAIN_NAME: &str = "ethereum-l1";

pub fn load_or_create_services_config() -> Result<Config, Box<dyn Error>> {
    let mut config_path = get_dot_monea_global_path();
    config_path.push("services.yaml");

    if !config_path.exists() {
        // Create default services.yaml config
        let default_manager_config = Config {
            enclave: Enclave {
                id: String::from("monea-enclave"),
                chains: {
                    let mut chains = HashMap::new();
                    chains.insert(
                        DEFAULT_CHAIN_NAME.to_string(),
                        Chain {
                            name: DEFAULT_CHAIN_NAME.to_string(),
                            services: Vec::new(),
                        },
                    );
                    chains
                },
            },
        };

        // Ensure the directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write default config
        let yaml = serde_yaml::to_string(&default_manager_config)?;
        fs::write(&config_path, yaml)?;

        Ok(default_manager_config)
    } else {
        // Load existing config
        let config_str = fs::read_to_string(&config_path)?;
        Ok(serde_yaml::from_str(&config_str)?)
    }
}

pub fn update_config(config: &mut Config, services: Vec<Service>) -> Result<(), Box<dyn Error>> {
    for service in services {
        for chain in config.enclave.chains.values_mut() {
            if let Some(existing_service) =
                chain.services.iter_mut().find(|s| s.name == service.name)
            {
                existing_service.uuid = service.uuid;
                existing_service.ports = service.ports;
                break;
            }
        }
    }

    Ok(())
}
