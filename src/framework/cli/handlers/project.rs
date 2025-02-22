use std::fs;
use std::path::PathBuf;
use crate::framework::cli::error::CliError;
use console::style;
use std::process::Command;

pub fn create_project(name: &str) -> Result<(), CliError> {
    let project_dir = PathBuf::from(name);
    
    // Create project directory
    fs::create_dir_all(&project_dir)
        .map_err(|e| CliError::IoError(e))?;

    // Create basic project structure
    let dirs = [
        "src/app/controllers",
        "src/app/models",
        "src/app/views",
        "src/app/middleware",
        "src/config",
        "src/routes",
        "src/public",
        "src/resources/js",
        "src/resources/css",
        "migrations",
    ];

    for dir in dirs.iter() {
        fs::create_dir_all(project_dir.join(dir))
            .map_err(|e| CliError::IoError(e))?;
    }

    // Create basic files with minimal content
    let files = [
        ("src/main.rs", "fn main() { println!(\"Hello, world!\"); }"),
        ("src/lib.rs", "// Library code goes here"),
        ("Cargo.toml", "[package]\nname = \"project\"\nversion = \"0.1.0\"\nedition = \"2021\""),
        (".env", "# Environment variables go here"),
        (".gitignore", "/target\n/Cargo.lock"),
        ("README.md", "# Project\n\nA new Ruskit project."),
    ];

    for (path, content) in files.iter() {
        fs::write(project_dir.join(path), content)
            .map_err(|e| CliError::IoError(e))?;
    }

    println!("Created new Ruskit project: {}", name);
    Ok(())
}

pub fn create_new_project(name: &str) -> Result<(), CliError> {
    println!("Creating new Ruskit project: {}", style(name).cyan());
    
    // Check if cargo-generate is installed
    let cargo_generate_check = Command::new("cargo-generate")
        .arg("--help")
        .output();
    
    if cargo_generate_check.is_err() {
        println!("{}", style("cargo-generate is not installed. Installing...").yellow());
        Command::new("cargo")
            .arg("install")
            .arg("cargo-generate")
            .status()
            .map_err(|e| CliError::IoError(e))?;
    }
    
    // Run cargo-generate with the GitHub template
    let status = Command::new("cargo-generate")
        .arg("generate")
        .arg("--git")
        .arg("https://github.com/dayemsiddiqui/ruskit")
        .arg("--branch")
        .arg("main")
        .arg("--name")
        .arg(name)
        .arg("--force")
        .env("CARGO_GENERATE_VALUE_PROJECT_NAME", name)
        .env("CARGO_GENERATE_VALUE_CRATE_NAME", name.to_lowercase())
        .env("CARGO_GENERATE_VALUE_BIN_NAME", name.to_lowercase())
        .status()
        .map_err(|e| CliError::IoError(e))?;
    
    if !status.success() {
        return Err(CliError::ProjectCreationError("Failed to generate project".into()));
    }
    
    println!("\n{} Project created successfully!", style("✓").green());
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  cargo make dev    # Start the development server");
    
    Ok(())
}

pub fn run_project(name: &str) -> Result<(), CliError> {
    create_project(name)?;
    Ok(())
} 