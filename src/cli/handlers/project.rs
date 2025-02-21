use crate::cli::error::CliError;
use console::style;
use std::process::Command;

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