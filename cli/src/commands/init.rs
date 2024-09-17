use clap::Args;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[clap()]
    pub project_path: String,

    #[arg(long, short = 'n')]
    pub name: Option<String>,
}

// Define monea config file content as a function
fn get_monea_config_content(name: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0"
}}"#,
        name
    )
}

pub fn init(args: InitArgs) -> Result<(), Box<dyn Error>> {
    let project_path = Path::new(&args.project_path);
    let project_name = args.name.unwrap_or_else(|| "Monea Rollup".to_string());
    let monea_config_content = get_monea_config_content(&project_name);

    // Check if monea.config.json already exists
    let config_file_path = project_path.join("monea.config.json");
    if config_file_path.exists() {
        return Err(format!(
            "monea.config.json already exists at {}",
            config_file_path.display()
        )
        .into());
    }

    if project_path.as_os_str() == "." || project_path.is_dir() {
        create_file(project_path, "monea.config.json", &monea_config_content)?;
    } else if !project_path.exists() {
        fs::create_dir_all(project_path)?;
        create_project_files(project_path, &monea_config_content)?;
    } else {
        return Err(format!(
            "'{}' already exists and is not a directory",
            args.project_path
        )
        .into());
    }

    println!("Project initialized successfully at: {:?}", project_path);
    Ok(())
}

fn create_project_files(
    project_path: &Path,
    monea_config_content: &str,
) -> Result<(), Box<dyn Error>> {
    // Define the files to be created and their contents
    let files = vec![
        (
            "README.md",
            "# My Monea Project\n\nWelcome to your new Monea project!",
        ),
        (
            "main.rs",
            "fn main() {\n    println!(\"Hello, Monea!\");\n}",
        ),
        ("monea.config.json", monea_config_content),
    ];

    for (file_name, content) in files {
        create_file(project_path, file_name, content)?;
    }

    Ok(())
}

fn create_file(project_path: &Path, file_name: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let file_path = project_path.join(file_name);
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
