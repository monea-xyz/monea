use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn create_temp_json_file<T: Serialize>(
    data: &T,
    filename: Option<&str>,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut config_dir = dirs::home_dir().expect("Unable to find home directory");
    config_dir.push(".monea");

    fs::create_dir_all(&config_dir)?;

    let temp_json_path = if let Some(name) = filename {
        config_dir.join(name)
    } else {
        // generate a random filename using mktemp
        let output = Command::new("mktemp")
            .arg("-p")
            .arg(&config_dir)
            .arg("temp_XXXXXXXX.json")
            .output()?;
        PathBuf::from(String::from_utf8(output.stdout)?.trim())
    };

    // use serde_json to serialize the data with proper indentation
    let json_content = serde_json::to_string_pretty(data)?;

    fs::write(&temp_json_path, json_content)?;

    Ok(temp_json_path)
}
