use std::fs;
use ts_rs::TS;

pub mod user;
pub mod about;
pub mod post;
pub use about::AboutPageProps;
pub use user::{UserResponse, CreateUserRequest, UserListResponse};
pub use post::{PostResponse, CreatePostRequest, PostListResponse};

pub fn export_all_types(output_file: &str) -> Result<(), ts_rs::ExportError> {
    // Create a buffer for all types
    let mut buffer = String::new();
    
    // Export types that have #[ts(export)] attribute
    buffer.push_str(&AboutPageProps::export_to_string()?);
    buffer.push('\n');
    buffer.push('\n');
    
    // Write all types at once
    fs::write(output_file, buffer)?;
    Ok(())
}
