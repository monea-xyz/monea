use monea_functions::{start_l1::start_l1, start_l2::start_l2};
use monea_manager::project_config::MoneaProjectConfig;
use monea_manager::MoneaManager;
use monea_utils::path_helper;
use std::error::Error;
use std::path::Path;

pub fn run_handler(relative_project_path: Option<&str>) -> Result<(), Box<dyn Error>> {
    let relative_project_path_as_path = relative_project_path
        .map(Path::new)
        .unwrap_or_else(|| Path::new(""));

    let mut manager =
        MoneaManager::with_project(&relative_project_path_as_path.to_path_buf(), true)?;

    let project_config = manager.project_config().unwrap();

    println!("project_config: {:#?}", project_config);

    // hardcoded service names for each chain
    // TODO
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
    // let l2_result = start_l2(&kurtosis_package_path);
    // if let Err(e) = l2_result {
    //     return Err(e);
    // }

    manager
        .services
        .parse_and_save_services_config(chain_services_to_parse)?;

    println!("Configuration updated successfully.");

    Ok(())
}
