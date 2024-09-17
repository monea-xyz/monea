use fs_extra::dir::CopyOptions;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Copy Kurtosis binary
    let kurtosis_bin = match target_os.as_str() {
        "linux" => "kurtosis-linux",
        "macos" => "kurtosis-macos",
        "windows" => "kurtosis-windows.exe",
        _ => panic!("Unsupported OS"),
    };

    fs_extra::copy_items(
        &[Path::new("bin").join(kurtosis_bin)],
        Path::new(&out_dir),
        &CopyOptions::new(),
    )
    .expect("Failed to copy Kurtosis binary");

    // Copy Kurtosis package
    fs_extra::copy_items(
        &[Path::new("kurtosis")],
        Path::new(&out_dir),
        &CopyOptions::new(),
    )
    .expect("Failed to copy Kurtosis package");
}
