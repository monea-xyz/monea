use crate::functions::{
    manager_save::manager_save_parsed_services, start_l1::start_l1, start_l2::start_l2,
};
use monea_utils::{path_helper, project_config::MoneaProjectConfig};
use std::error::Error;
use std::path::Path;

pub fn run_handler(config_path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    MoneaProjectConfig::verify(config_path.expect("config_path is None"))?;

    // hardcoded service names for each chain
    let chain_services_to_parse: Vec<(&str, Vec<&str>)> = vec![
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
    let l1_result = start_l1(&kurtosis_package_path);
    if let Err(e) = l1_result {
        return Err(e);
    }

    // start the l2 network
    let l2_result = start_l2(&kurtosis_package_path);
    if let Err(e) = l2_result {
        return Err(e);
    }

    manager_save_parsed_services(chain_services_to_parse)?;

    println!("Configuration updated successfully.");

    Ok(())
}
