use std::fs;
use ts_rs::TS;

pub mod user;
pub mod about;
pub mod post;
pub use about::AboutPageProps;
pub use user::{UserResponse, CreateUserRequest, UserListResponse};
pub use post::{PostResponse, CreatePostRequest, PostListResponse};

// Define a struct to hold TS export function pointers
pub struct TsExporter {
    pub export_fn: fn() -> Result<String, ts_rs::ExportError>,
}

// Register the inventory collection for TsExporter items
inventory::collect!(TsExporter);

// Macro to register a type for TS export; types with #[ts(export)] attribute should call this macro
#[macro_export]
macro_rules! register_ts {
    ($type:ty) => {
        inventory::submit! {
            crate::app::dtos::TsExporter {
                export_fn: <$type as ts_rs::TS>::export_to_string,
            }
        }
    };
}

pub fn export_all_types(output_file: &str) -> Result<(), ts_rs::ExportError> {
    // Create a buffer for all types
    let mut buffer = String::new();

    let mut types = vec![];

    // Dynamically read all structs in the current module with #[ts(export)] attribute using the inventory registry
    for exporter in inventory::iter::<TsExporter> {
         types.push((exporter.export_fn)()?);
    }
    
    // Export types that have #[ts(export)] attribute
    for type_str in types { 
        buffer.push_str(&type_str);
        buffer.push('\n');
    }
    
    // Write all types at once
    fs::write(output_file, buffer)?;
    Ok(())
}
