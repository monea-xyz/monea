use monea_utils::path_helper;
use std::{error::Error, path::Path, process::Command};

pub fn start_l1(kurtosis_package_path: &Path) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}
