use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::process::Command;

// We'll need to import or redefine these types and functions
use monea_manager::{parse_enclave_inspect_stdout, Manager, Service};
use monea_utils::path_helper;

pub fn run_handler(config_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let config_path = config_path.unwrap_or_else(|| ".".to_string());
    let config_path = Path::new(&config_path).join("monea.config.yaml");

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
                "op-cl-1-op-node-op-reth-op-kurtosis",
                "op-el-1-op-reth-op-node-op-kurtosis",
                "op-batcher-op-kurtosis",
                // there is no op-proposer in OP Stack anymore
                // "op-proposer-op-kurtosis",
            ],
        ),
    ];

    let kurtosis_package_path = path_helper::get_kurtosis_package_path();

    // start the l1 network
    let mut l1_command = Command::new("kurtosis");
    l1_command.arg("run");
    l1_command.arg(&kurtosis_package_path);
    l1_command.arg("--main-file");
    l1_command.arg("start_l1.star");
    l1_command.arg("--enclave");
    l1_command.arg("monea-enclave");

    let l1_command_status = l1_command.status()?;

    if !l1_command_status.success() {
        return Err("Kurtosis run command failed for l1".into());
    }

    // copy l1_config file from the enclave into the kurtosis package directory
    Command::new("kurtosis")
        .arg("files")
        .arg("download")
        .arg("monea-enclave")
        .arg("l1_config")
        .arg(path_helper::get_kurtosis_package_path())
        .output()?;

    // start the l2 network
    let mut l2_command = Command::new("kurtosis");
    l2_command.arg("run");
    l2_command.arg(&kurtosis_package_path);
    l2_command.arg("--main-file");
    l2_command.arg("start_opstack.star");
    l2_command.arg("--args-file");
    l2_command.arg(Path::new(&kurtosis_package_path).join("network_params.yaml"));
    l2_command.arg("--enclave");
    l2_command.arg("monea-enclave");

    let l2_command_status = l2_command.status()?;

    if !l2_command_status.success() {
        return Err("Kurtosis run command failed for l2".into());
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
