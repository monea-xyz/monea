use dotenv::dotenv;
use reqwest;
use serde_json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    dotenv().ok();
    let profile = env::var("PROFILE").unwrap();
    let cli_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let engine_src = Path::new(&cli_dir).join("..").join("engine");
    let engine_dest = Path::new(&cli_dir)
        .join("..")
        .join("target")
        .join(&profile)
        .join("engine");
    // let kurtosis_dest = Path::new(&cli_dir).join("..").join("target").join(&profile);

    println!("engine_src: {}", engine_src.display());
    println!("engine_dest: {}", engine_dest.display());

    // Copy the engine files
    copy_engine(&engine_src, &engine_dest);

    // Download and build Kurtosis
    // download_and_build_kurtosis(&kurtosis_dest);

    println!("cargo:rerun-if-changed=../engine");
}

fn copy_engine(engine_src: &Path, engine_dest: &Path) {
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
}

fn download_and_build_kurtosis(dest_dir: &Path) {
    let github_api_key = env::var("GITHUB_API_KEY").expect("GITHUB_API_KEY must be set");
    let kurtosis_repo = "kurtosis-tech/kurtosis";
    let latest_release_url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        kurtosis_repo
    );

    // Get the latest release information
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&latest_release_url)
        .header("Authorization", format!("token {}", github_api_key))
        .header("User-Agent", "monea-cli")
        .send()
        .expect("Failed to fetch latest release information");

    let release_json: serde_json::Value = response.json().expect("Failed to parse JSON response");
    let tarball_url = release_json["tarball_url"]
        .as_str()
        .expect("Failed to get tarball_url");

    println!("Tarball URL: {}", tarball_url);

    // Download the tarball
    let tar_gz_path = dest_dir.join("kurtosis.tar.gz");
    let mut response = client
        .get(tarball_url)
        .header("Authorization", format!("token {}", github_api_key))
        .header("User-Agent", "monea-cli")
        .send()
        .expect("Failed to download Kurtosis release");

    println!("Downloading Kurtosis source from: {}", tarball_url);
    let mut file = fs::File::create(&tar_gz_path).expect("Failed to create tar.gz file");
    std::io::copy(&mut response, &mut file).expect("Failed to write tar.gz file");

    // Extract the tarball
    let extract_dir = dest_dir.join("kurtosis_source");
    fs::create_dir_all(&extract_dir).expect("Failed to create extraction directory");

    Command::new("tar")
        .arg("-xzf")
        .arg(&tar_gz_path)
        .arg("-C")
        .arg(&extract_dir)
        .arg("--strip-components=1")
        .status()
        .expect("Failed to extract Kurtosis source");

    // Build Kurtosis using Nix
    println!("Building Kurtosis...");

    // TODO i don't think building nix within this script is the way to go,
    // ? we need to run our entire environment from within nix at the start and
    // ?just dry-call the build.sh script directly right here
    let build_status = Command::new("nix")
        .arg("develop")
        .arg("-c")
        .arg("./scripts/build.sh")
        .current_dir(&extract_dir)
        .status()
        .expect("Failed to build Kurtosis");

    if !build_status.success() {
        panic!("Kurtosis build failed");
    }

    // Copy the built binary to the target/debug directory
    let binary_path = extract_dir
        .join("cli")
        .join("cli")
        .join("build")
        .join("kurtosis");
    let target_path = dest_dir.join("kurtosis");
    fs::copy(binary_path, target_path).expect("Failed to copy Kurtosis binary");

    // Clean up
    fs::remove_file(tar_gz_path).expect("Failed to remove Kurtosis tar.gz file");
    fs::remove_dir_all(extract_dir).expect("Failed to remove Kurtosis source directory");

    println!("Successfully built and copied Kurtosis binary");
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
