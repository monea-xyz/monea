use monea_manager::project_config::MoneaProjectConfig;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn init_handler(
    project_path: &str,
    name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = Path::new(project_path);

    if project_path.as_os_str() == "." || project_path.is_dir() {
        MoneaProjectConfig::new(project_path, name)?;
    } else if !project_path.exists() {
        fs::create_dir_all(project_path)?;
        create_project_files(project_path, name)?;
        // Initialize Git repository
        init_git_repo(project_path)?;
    } else {
        return Err(format!(
            "'{}' already exists and is not a directory",
            project_path.display()
        )
        .into());
    }

    println!("Project initialized successfully at: {:?}", project_path);
    Ok(())
}

fn create_project_files(project_path: &Path, name: Option<String>) -> Result<(), Box<dyn Error>> {
    // Create monea.config.yaml
    MoneaProjectConfig::new(project_path, name)?;

    // Create README.md
    let readme_content = "# My Monea Project\n\nWelcome to your new Monea project!";
    let readme_path = project_path.join("README.md");
    let mut readme_file = fs::File::create(readme_path)?;
    readme_file.write_all(readme_content.as_bytes())?;

    Ok(())
}

fn init_git_repo(project_path: &Path) -> Result<(), Box<dyn Error>> {
    Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .output()
        .map_err(|e| format!("Failed to initialize Git repository: {}", e))?;
    println!("Initialized Git repository");
    Ok(())
}
