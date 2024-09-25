use dirs::home_dir;
use std::path::PathBuf;

pub const ETHEREUM_L1_CHAIN_NAME: &str = "ethereum-l1";
pub const OPTIMISM_L2_CHAIN_NAME: &str = "optimism-l2";

pub fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Unable to find home directory");
    path.push(".monea");
    path.push("manager.yaml");
    path
}
