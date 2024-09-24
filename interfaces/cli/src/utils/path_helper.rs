use std::env;
use std::path::PathBuf;

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
    path.push("engine");
    path
}
