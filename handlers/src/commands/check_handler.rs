use monea_utils::project_config::MoneaProjectConfig;
use std::path::Path;

pub fn check_handler(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match MoneaProjectConfig::verify(Path::new(config_path)) {
        Ok(_) => {
            println!("Configuration is valid.");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
