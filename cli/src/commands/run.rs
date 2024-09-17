use crate::utils::path_helper;
use clap::Args;
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

    // TODO copy kurtosis binary next to monea-cli binary during build and run that
    // let kurtosis_binary_path = path_helper::get_kurtosis_binary_path();
    let kurtosis_package_path = path_helper::get_kurtosis_package_path();

    let mut command = Command::new("kurtosis");
    command.arg("run");
    command.arg(&kurtosis_package_path);
    command.arg("--args-file");
    command.arg(Path::new(&kurtosis_package_path).join("network_params.yaml"));

    let status = command.status()?;

    if !status.success() {
        return Err("Kurtosis run command failed".into());
    }

    println!("Kurtosis run command executed successfully");
    Ok(())
}
