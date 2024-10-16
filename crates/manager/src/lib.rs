use monea_utils::path_helper;
use project_config::MoneaProjectConfig;
use services::{Config as ServicesConfig, Services};
use std::path::PathBuf;

pub mod kurtosis_params;
pub mod project_config;
pub mod services;

pub struct MoneaManager {
    global_dot_dir_abs_path: PathBuf,
    local_dot_dir_abs_path: Option<PathBuf>,
    pub project_config: Option<MoneaProjectConfig>,
    pub services: Services,
}

impl MoneaManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let global_dot_dir_abs_path = Self::get_global_dot_dir_abs_path()?;
        let services = services::Services::new()?;

        Ok(Self {
            global_dot_dir_abs_path,
            local_dot_dir_abs_path: None,
            project_config: None,
            services,
        })
    }

    pub fn with_project(
        project_path: &PathBuf,
        verify_project_config: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut manager = Self::new()?;
        manager.load_project(project_path, verify_project_config)?;
        Ok(manager)
    }

    pub fn load_project(
        &mut self,
        project_path: &PathBuf,
        verify_project_config: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // use path_helper to get the absolute path for the local .monea directory
        self.local_dot_dir_abs_path = Some(path_helper::get_dot_monea_local_path(Some(
            project_path.to_str().unwrap(),
        )));

        if verify_project_config {
            MoneaProjectConfig::verify(project_path)?;
        }

        self.project_config = Some(MoneaProjectConfig::from_file(project_path)?);
        Ok(())
    }

    pub fn project_config(&self) -> Option<&MoneaProjectConfig> {
        self.project_config.as_ref()
    }

    pub fn services_config(&self) -> &ServicesConfig {
        &self.services.config
    }

    pub fn global_dot_dir_abs_path(&self) -> &PathBuf {
        &self.global_dot_dir_abs_path
    }

    pub fn local_dot_dir_abs_path(&self) -> Option<&PathBuf> {
        self.local_dot_dir_abs_path.as_ref()
    }

    fn get_global_dot_dir_abs_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(path_helper::get_dot_monea_global_path())
    }

    // Add more methods for managing other config files as needed
}
