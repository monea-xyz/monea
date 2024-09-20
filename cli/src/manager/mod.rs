use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod config;
pub mod parse_enclave_inspect_stdout;

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

pub struct Manager {
    pub config: Config,
}

impl Manager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = config::load_or_create_config()?;
        Ok(Manager { config })
    }

    pub fn save_config(&self) -> Result<(), Box<dyn Error>> {
        let updated_yaml = serde_yaml::to_string(&self.config)?;
        fs::write(config::get_config_path(), updated_yaml)?;
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
}
