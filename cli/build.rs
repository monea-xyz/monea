// use fs_extra::dir::CopyOptions;
// use std::env;
// use std::path::Path;

// fn main() {
//     // Tell Cargo that if the given file changes, to rerun this build script.
//     println!("cargo:rerun-if-changed=engine");

//     let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

//     // Get the directory of the current executable
//     let mut out_dir = env::current_exe().expect("Failed to get current executable path");
//     out_dir.pop(); // Remove the executable name to get the directory

//     // TODO add kurtosis binary copy

//     // // Copy Kurtosis binary
//     // let kurtosis_bin = match target_os.as_str() {
//     //     "linux" => "kurtosis-linux",
//     //     "macos" => "kurtosis-macos",
//     //     "windows" => "kurtosis-windows.exe",
//     //     _ => panic!("Unsupported OS"),
//     // };

//     // println!("Copying Kurtosis CLI binary: {}", kurtosis_bin);

//     // fs_extra::copy_items(
//     //     &[Path::new("bin").join(kurtosis_bin)],
//     //     &out_dir,
//     //     &CopyOptions::new(),
//     // )
//     // .expect("Failed to copy Kurtosis binary");

//     // Copy Kurtosis package
//     fs_extra::copy_items(&[Path::new("engine")], &out_dir, &CopyOptions::new())
//         .expect("Failed to copy Monea Engine Kurtosis package");
// }

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let cli_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let engine_src = Path::new(&cli_dir).join("..").join("engine");
    let engine_dest = Path::new(&cli_dir)
        .join("..")
        .join("target")
        .join(profile)
        .join("engine");

    println!("engine_src: {}", engine_src.display());
    println!("engine_dest: {}", engine_dest.display());

    // Check if source directory exists
    if !engine_src.exists() {
        eprintln!(
            "Error: Monea Engine source directory does not exist: {}",
            engine_src.display()
        );
        std::process::exit(1);
    }

    // Remove previous copy of engine if it exists
    if engine_dest.exists() {
        fs::remove_dir_all(&engine_dest).unwrap_or_else(|e| {
            eprintln!(
                "Warning: Failed to remove existing engine directory from target: {}",
                e
            );
        });
    }

    // Create the destination directory
    fs::create_dir_all(&engine_dest).unwrap_or_else(|e| {
        eprintln!("Error: Failed to create engine directory in target: {}", e);
        std::process::exit(1);
    });

    // Copy the engine files
    match copy_dir_all(&engine_src, &engine_dest) {
        Ok(_) => println!("Successfully copied engine files to target"),
        Err(e) => {
            eprintln!("Error: Failed to copy engine files: {}", e);
            std::process::exit(1);
        }
    }

    println!("cargo:rerun-if-changed=../engine");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    println!("Copying directory: {}", src.display());
    println!("To directory: {}", dst.display());

    if !src.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Source is not a directory: {}", src.display()),
        ));
    }

    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.join(entry.file_name()))?;
        } else {
            println!(
                "Copying file: {} to {}",
                entry.path().display(),
                dst.join(entry.file_name()).display()
            );
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
