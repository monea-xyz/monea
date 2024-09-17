use crate::commands::InitArgs;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn init(args: InitArgs) -> Result<(), Box<dyn Error>> {
    let project_path = Path::new(&args.project_name);

    if project_path.as_os_str() == "." {
        create_project_file(project_path)?;
    } else if !project_path.exists() {
        fs::create_dir_all(project_path)?;
        create_project_files(project_path)?;
    } else if project_path.is_dir() {
        create_project_file(project_path)?;
    } else {
        return Err(format!(
            "'{}' already exists and is not a directory",
            args.project_name
        )
        .into());
    }

    println!("Project initialized successfully at: {:?}", project_path);
    Ok(())
}

fn create_project_files(project_path: &Path) -> Result<(), Box<dyn Error>> {
    // Define the files to be created and their contents
    let files = vec![
        (
            "README.md",
            "# My Monea Project\n\nWelcome to your new Monea project!",
        ),
        (
            "config.json",
            "{\n  \"name\": \"my_monea_project\",\n  \"version\": \"0.1.0\"\n}",
        ),
        (
            "main.rs",
            "fn main() {\n    println!(\"Hello, Monea!\");\n}",
        ),
    ];

    for (file_name, content) in files {
        let file_path = project_path.join(file_name);
        let mut file = fs::File::create(file_path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

fn create_project_file(project_path: &Path) -> Result<(), Box<dyn Error>> {
    let file_path = project_path.join("monea_project.json");
    let content = "{\n  \"name\": \"monea_project\",\n  \"version\": \"0.1.0\"\n}";
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
