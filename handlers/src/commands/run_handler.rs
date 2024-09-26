use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::process::Command;

// We'll need to import or redefine these types and functions
use monea_manager::{parse_enclave_inspect_stdout, Manager, Service};
use monea_utils::path_helper;

pub fn run_handler(project_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let project_path = project_path.unwrap_or_else(|| ".".to_string());
    let config_path = Path::new(&project_path).join("monea.config.yaml");

    if !config_path.exists() {
        return Err("monea.config.yaml not found. Please run 'monea init' first.".into());
    }

    // Hardcoded service names for each chain
    let chain_services: Vec<(&str, Vec<&str>)> = vec![
        (
            "ethereum-l1",
            vec![
                "cl-1-lighthouse-geth",
                "el-1-geth-lighthouse",
                "vc-1-geth-lighthouse",
            ],
        ),
        (
            "optimism-l2",
            vec![
                "op-cl-1-op-node-op-reth",
                "op-el-1-op-reth-op-node",
                "op-proposer",
                "op-batcher",
            ],
        ),
    ];

    let kurtosis_package_path = path_helper::get_kurtosis_package_path();

    let mut command = Command::new("kurtosis");
    command.arg("run");
    command.arg(&kurtosis_package_path);
    command.arg("--args-file");
    command.arg(Path::new(&kurtosis_package_path).join("network_params.yaml"));
    command.arg("--enclave");
    command.arg("monea-enclave");

    let status = command.status()?;

    if !status.success() {
        return Err("Kurtosis run command failed".into());
    }

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

    println!("Configuration updated successfully.");

    Ok(())
}
