use std::{fs, path::PathBuf};

use monea_utils::path_helper;

use crate::config;

use super::model::UserCredentials;

#[derive(Debug, Clone)]
pub struct CredentialsStoreClient {
    root_path: PathBuf,
    file_name: String,
    file_path: PathBuf,
}

impl CredentialsStoreClient {
    pub fn new() -> Result<CredentialsStoreClient, std::io::Error> {
        let root_path = path_helper::get_dot_monea_global_path(); // TODO: this can panic
        let file_path = root_path.join(config::CREDENTIALS_FILE);

        let client = CredentialsStoreClient {
            root_path: root_path,
            file_path: file_path,
            file_name: config::CREDENTIALS_FILE.to_string(),
        };

        CredentialsStoreClient::_ensure_credentials_file_exists(client.clone())?;

        Ok(client)
    }

    pub fn set_credentials(&self, credentials: UserCredentials) -> Result<(), std::io::Error> {
        let serialized_credentials = serde_json::to_string(&credentials).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error serializing credentials: {}", err),
            )
        })?;

        fs::write(self.file_path.as_path(), serialized_credentials).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error writing credentials to file: {}", err),
            )
        })
    }

    pub fn get_credentials(&self) -> Result<UserCredentials, std::io::Error> {
        let file_contents = fs::read_to_string(self.file_path.as_path()).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error reading credentials from file: {}", err),
            )
        })?;

        let credentials: UserCredentials = serde_json::from_str(&file_contents).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error deserializing credentials: {}", err),
            )
        })?;

        Ok(credentials)
    }

    fn _ensure_credentials_file_exists(
        client: CredentialsStoreClient,
    ) -> Result<(), std::io::Error> {
        if !client.root_path.exists() {
            fs::create_dir_all(client.root_path).map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error creating credentials directory: {}", err),
                )
            })?;
        }
        let path = client.file_path.as_path();

        if !path.exists() {
            fs::write(path, "".as_bytes()).map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error creating credentials file: {}", err),
                )
            })?;
        }

        Ok(())
    }
}
