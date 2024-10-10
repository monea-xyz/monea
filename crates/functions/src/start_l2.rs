use std::{error::Error, path::Path, process::Command};

pub fn start_l2(kurtosis_package_path: &Path) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
