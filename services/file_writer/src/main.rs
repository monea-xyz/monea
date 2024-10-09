use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// NOTE: This is a standalone script used by the Kurtosis package to write an enclave artifact
// it's logic should be pure and maximally simple

// Example Steps:
// 1. run the below plan.run_sh to write the l1_config.json file to the container's filesystem
// 2. upload it to the enclave artifact with plan.run_sh (via store/StoreSpec argument)
// 3. download it from anywhere by running: `kurtosis files download <enclave_name> <artifact_name> <destination_path>`
// 4. then run read_file in another starlark script to access that data

// plan.run_sh(
//     name = "l1_config_file_writer",
//     image = "file-writer"
//     store = [
//       StoreSpec(
//         name = "l1_config_file",
//         path = "/l1_config.yaml"
//       )
//     ]

fn main() -> std::io::Result<()> {
    // get command-line arguments
    let args: Vec<String> = env::args().collect();

    // check if we have the correct number of arguments
    if args.len() != 3 {
        eprintln!("Usage: {} <stringified_data> <filename>", args[0]);
        std::process::exit(1);
    }

    // extract the stringified data and filename from arguments
    let stringified_data = &args[1];
    let filename = &args[2];

    // create a path for the new file in the current working directory
    let path = Path::new(filename);

    // create (or open) the file
    let mut file = File::create(path)?;

    // write the stringified data to the file
    file.write_all(stringified_data.as_bytes())?;

    println!(
        "File '{}' has been created and data has been written successfully.",
        filename
    );

    Ok(())
}
