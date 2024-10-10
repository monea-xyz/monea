use super::{ConfigError, MoneaProjectConfig};
use std::{fs, path::PathBuf};

// get the path to the project config file, either .yaml or .yml
pub fn get_project_config_file_path(project_config_path: &PathBuf) -> Result<PathBuf, ConfigError> {
    let path = project_config_path
        .canonicalize()
        .map_err(|_| ConfigError::NotFound)?;

    // check if the path already ends with monea.config.yaml or monea.config.yml
    if path.ends_with("monea.config.yaml") || path.ends_with("monea.config.yml") {
        return Ok(path);
    }

    let yaml_path = path.join("monea.config.yaml");
    let yml_path = path.join("monea.config.yml");

    if yaml_path.exists() {
        Ok(yaml_path)
    } else if yml_path.exists() {
        Ok(yml_path)
    } else {
        Err(ConfigError::NotFound)
    }
}

pub fn read_project_config_file_as_string(
    project_config_path: &PathBuf,
) -> Result<String, ConfigError> {
    let path = get_project_config_file_path(project_config_path)?;
    let config_contents = fs::read_to_string(path)?;
    Ok(config_contents)
}

// read the project config file and parse it into a MoneaProjectConfig struct
pub fn read_project_config_file_as_struct(
    project_config_path: &PathBuf,
) -> Result<MoneaProjectConfig, ConfigError> {
    let config_contents = read_project_config_file_as_string(project_config_path)?;
    let config = serde_yaml::from_str(&config_contents)?;
    Ok(config)
}
