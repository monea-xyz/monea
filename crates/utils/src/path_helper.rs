use std::env;
use std::path::{Path, PathBuf};

pub fn get_kurtosis_binary_path() -> PathBuf {
    let mut path = env::current_exe().expect("Failed to get current executable path");
    path.pop();
    path.push("kurtosis");
    #[cfg(target_os = "windows")]
    path.set_extension("exe");
    path
}

pub fn get_kurtosis_package_path() -> PathBuf {
    let mut path = env::current_exe().expect("Failed to get current executable path");
    path.pop();
    path.push("kurtosis-pkg");
    path
}

pub fn get_dot_monea_global_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    path.push(".monea");
    path
}

pub fn get_project_absolute_path(relative_config_path: Option<&str>) -> PathBuf {
    let config_path = match relative_config_path {
        Some(path) => {
            let absolute_path =
                std::path::absolute(Path::new(path)).expect("Failed to get absolute path");
            PathBuf::from(absolute_path)
        }
        None => env::current_dir().expect("Failed to get current directory"),
    };

    config_path
}

pub fn get_dot_monea_local_path(relative_config_path: Option<&str>) -> PathBuf {
    let project_absolute_path = get_project_absolute_path(relative_config_path);
    project_absolute_path.join(".monea")
}

pub fn path_to_relative_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if path.is_relative() {
        // if it's already relative, normalize and return
        path.normalize()
    } else {
        // if it's absolute, convert to relative
        let current_dir = env::current_dir().expect("Failed to get current directory");
        path.strip_prefix(current_dir)
            .expect("Failed to strip prefix")
            .normalize()
    }
}

pub fn path_to_absolute_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if path.is_absolute() {
        // if it's already absolute, normalize and return
        path.normalize()
    } else {
        // if it's relative, convert to absolute
        let current_dir = env::current_dir().expect("Failed to get current directory");
        current_dir.join(path).normalize()
    }
}

// helper function to normalize paths
trait PathNormalize {
    fn normalize(&self) -> PathBuf;
}

impl PathNormalize for Path {
    fn normalize(&self) -> PathBuf {
        let mut components = self.components().peekable();
        let mut ret = if let Some(c @ std::path::Component::Prefix(..)) = components.peek().cloned()
        {
            components.next();
            PathBuf::from(c.as_os_str())
        } else {
            PathBuf::new()
        };

        for component in components {
            match component {
                std::path::Component::Prefix(..) => unreachable!(),
                std::path::Component::RootDir => {
                    ret.push(component.as_os_str());
                }
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    ret.pop();
                }
                std::path::Component::Normal(c) => {
                    ret.push(c);
                }
            }
        }
        ret
    }
}
