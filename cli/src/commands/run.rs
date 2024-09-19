use crate::utils::manager::parse_enclave_inspect_stdout;
use crate::utils::manager::Manager;
use crate::utils::manager::Service;
use crate::utils::path_helper;
use clap::Args;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::process::Command;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg()]
    pub project_path: Option<String>,
}

pub fn run(args: RunArgs) -> Result<(), Box<dyn Error>> {
    let project_path = args.project_path.unwrap_or_else(|| ".".to_string());
    let config_path = Path::new(&project_path).join("monea.config.json");

    if !config_path.exists() {
        return Err("monea.config.json not found. Please run 'monea init' first.".into());
    }

    // Hardcoded service names for each chain
    let chain_services = vec![
        (
            "ethereum-l1",
            vec![
                "cl-1-lighthouse-geth",
                "el-1-geth-lighthouse",
                "vc-1-geth-lighthouse",
            ],
        ),
        ("optimism-l2", vec!["op-node", "op-geth"]),
    ];

    let kurtosis_package_path = path_helper::get_kurtosis_package_path();

    let mut command = Command::new("kurtosis");
    command.arg("run");
    command.arg(&kurtosis_package_path);
    command.arg("--args-file");
    command.arg(Path::new(&kurtosis_package_path).join("network_params.yaml"));
    command.arg("--enclave");
    command.arg("monea-engine");

    let status = command.status()?;

    if !status.success() {
        return Err("Kurtosis run command failed".into());
    }

    // Now, let's inspect the enclave and parse the output
    let output = Command::new("kurtosis")
        .arg("enclave")
        .arg("inspect")
        .arg("monea-engine")
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
