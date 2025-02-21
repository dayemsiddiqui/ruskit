use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    DatabaseError(String),
    MigrationError(String),
    ProjectCreationError(String),
    FileNotFoundError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IoError(err) => write!(f, "IO Error: {}", err),
            CliError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            CliError::MigrationError(msg) => write!(f, "Migration Error: {}", msg),
            CliError::ProjectCreationError(msg) => write!(f, "Project Creation Error: {}", msg),
            CliError::FileNotFoundError(msg) => write!(f, "File Not Found: {}", msg),
        }
    }
}

impl Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
} 