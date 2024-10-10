use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

// We'll need to import or redefine these types and functions
use monea_manager::Manager;
use monea_utils::create_yaml_config::create_temp_yaml_file;
use monea_utils::path_helper;

pub fn stop_handler(project_path: Option<String>, layer: String) -> Result<(), Box<dyn Error>> {
    let project_path = project_path.unwrap_or_else(|| ".".to_string());
    let config_path = Path::new(&project_path).join("monea.config.yaml");

    if !config_path.exists() {
        return Err("monea.config.yaml not found. Please run 'monea init' first.".into());
    }

    let manager = Manager::new()?;
    let services = manager
        .get_services_by_chain(&layer)
        .ok_or_else(|| format!("Chain '{}' not found", layer))?;

    let service_names: Vec<String> = services.iter().map(|s| s.name.clone()).collect();

    // Create temporary YAML file using the new module
    let services_data = serde_yaml::Mapping::from_iter(vec![(
        serde_yaml::Value::String("services".to_string()),
        serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(vec![(
            serde_yaml::Value::String("to_stop".to_string()),
            serde_yaml::Value::Sequence(
                service_names
                    .into_iter()
                    .map(serde_yaml::Value::String)
                    .collect(),
            ),
        )])),
    )]);

    let temp_yaml_path = create_temp_yaml_file(&services_data, Some("temp_stop_services.yaml"))?;

    let current_dir = std::env::current_dir()?;

    let relative_path = pathdiff::diff_paths(&temp_yaml_path, &current_dir)
        .unwrap_or_else(|| temp_yaml_path.clone());

    let kurtosis_package_path = path_helper::get_kurtosis_package_path();

    let mut command = Command::new("kurtosis");
    command.arg("run");
    command.arg(&kurtosis_package_path);
    command.arg("--args-file");
    command.arg(relative_path);
    command.arg("--main-file");
    command.arg("stop_services.star");
    command.arg("--enclave");
    command.arg("monea-enclave");

    let status = command.status()?;

    // Clean up temporary file
    fs::remove_file(temp_yaml_path)?;

    if !status.success() {
        return Err("Kurtosis run command failed".into());
    }

    println!("Successfully stopped services for layer: {}", layer);

    Ok(())
}
