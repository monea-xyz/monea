use dirs::home_dir;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use super::{Chain, Config, Enclave, Service};

const DEFAULT_CHAIN_NAME: &str = "ethereum-l1";

pub fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Unable to find home directory");
    path.push(".monea");
    path.push("manager.yaml");
    path
}

pub fn load_or_create_config() -> Result<Config, Box<dyn Error>> {
    let config_path = get_config_path();

    println!("Config path: {}", config_path.display());

    if !config_path.exists() {
        // Create default manager.yaml config
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
