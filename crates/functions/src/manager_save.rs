use std::{collections::HashMap, error::Error, process::Command};

use monea_manager::{parse_enclave_inspect_stdout, Manager, Service};

pub fn manager_save_parsed_services(
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

    let mut manager = Manager::new()?;

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
                    manager.add_or_update_service(chain_name, parsed_service.clone())?;
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
    manager.save_config()?;

    Ok(())
}
