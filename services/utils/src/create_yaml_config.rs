use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn create_yaml_config<T: Serialize>(
    data: &T,
    filename: Option<&str>,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut config_dir = dirs::home_dir().expect("Unable to find home directory");
    config_dir.push(".monea");

    fs::create_dir_all(&config_dir)?;

    let temp_yaml_path = if let Some(name) = filename {
        config_dir.join(name)
    } else {
        // Generate a random filename using mktemp
        let output = Command::new("mktemp")
            .arg("-p")
            .arg(&config_dir)
            .arg("temp_XXXXXXXX.yaml")
            .output()?;
        PathBuf::from(String::from_utf8(output.stdout)?.trim())
    };

    // Use serde_yaml to serialize the data with proper indentation
    let yaml_content = serde_yaml::to_string(data)?;

    // Ensure proper indentation for list items
    let indented_content = yaml_content
        .lines()
        .map(|line| {
            if line.trim().starts_with('-') {
                format!("  {}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(&temp_yaml_path, indented_content)?;

    Ok(temp_yaml_path)
}
