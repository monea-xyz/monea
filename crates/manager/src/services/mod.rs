use monea_utils::constants;
use monea_utils::path_helper::get_dot_monea_global_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process::Command;

pub mod parse_enclave_inspect_stdout;
pub mod services_config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Port {
    pub name: String,
    pub internal: u16,
    pub external: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub uuid: String,
    pub ports: Vec<Port>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chain {
    pub name: String,
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enclave {
    pub id: String,
    pub chains: HashMap<String, Chain>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub enclave: Enclave,
}

pub struct Services {
    pub config: Config,
}

impl Services {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = services_config::load_or_create_services_config()?;
        Ok(Services { config })
    }

    pub fn save_config(&self) -> Result<(), Box<dyn Error>> {
        let updated_yaml = serde_yaml::to_string(&self.config)?;
        let mut config_path = get_dot_monea_global_path();
        config_path.push("services.yaml");
        fs::write(config_path, updated_yaml)?;
        Ok(())
    }

    pub fn get_all_services(&self) -> Vec<&Service> {
        self.config
            .enclave
            .chains
            .values()
            .flat_map(|chain| &chain.services)
            .collect()
    }

    pub fn get_services_by_chain(&self, chain_name: &str) -> Option<&Vec<Service>> {
        self.config
            .enclave
            .chains
            .get(chain_name)
            .map(|chain| &chain.services)
    }

    pub fn add_or_update_service(
        &mut self,
        chain_name: &str,
        new_service: Service,
    ) -> Result<(), Box<dyn Error>> {
        let chain = self
            .config
            .enclave
            .chains
            .entry(chain_name.to_string())
            .or_insert_with(|| Chain {
                name: chain_name.to_string(),
                services: Vec::new(),
            });

        if let Some(existing_service) = chain
            .services
            .iter_mut()
            .find(|s| s.name == new_service.name)
        {
            // Update only the fields that are different
            if existing_service.uuid != new_service.uuid {
                existing_service.uuid = new_service.uuid.clone();
            }
            // Update ports
            existing_service.ports = new_service.ports.clone();
        } else {
            // If the service doesn't exist, add it to the chain's services
            chain.services.push(new_service.clone());
        }

        Ok(())
    }

    pub fn parse_and_save_services_config(
        &mut self,
        chain_services: Vec<(&str, Vec<&str>)>,
    ) -> Result<(), Box<dyn Error>> {
        // Now, let's inspect the enclave and parse the output
        let output = Command::new("kurtosis")
            .arg("enclave")
            .arg("inspect")
            .arg("monea-enclave")
            .output()?;

        if !output.status.success() {
            return Err("Kurtosis enclave inspect command failed".into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        let parsed_services = parse_enclave_inspect_stdout::parse(&stdout)?;

        // Create a HashMap to easily look up parsed services by name
        let parsed_services_map: HashMap<String, Service> = parsed_services
            .into_iter()
            .map(|service| (service.name.clone(), service))
            .collect();

        // Iterate over the hardcoded chain_services
        for (chain_name, service_names) in chain_services {
            for service_name in service_names {
                match parsed_services_map.get(service_name) {
                    Some(parsed_service) => {
                        self.add_or_update_service(chain_name, parsed_service.clone())?;
                    }
                    None => {
                        println!(
                            "Warning: Service '{}' wasn't found in parsed stdout",
                            service_name
                        );
                    }
                }
            }
        }

        // Save the updated configuration
        self.save_config()?;

        Ok(())
    }
}
