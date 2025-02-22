use std::io;

#[derive(Debug)]
pub enum CliError {
    IoError(io::Error),
    Other(String),
    ProjectCreationError(String),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<String> for CliError {
    fn from(error: String) -> Self {
        CliError::Other(error)
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::IoError(e) => write!(f, "IO Error: {}", e),
            CliError::Other(e) => write!(f, "{}", e),
            CliError::ProjectCreationError(e) => write!(f, "Project Creation Error: {}", e),
        }
    }
}

impl std::error::Error for CliError {} 