//! This file is auto-generated after running database migrations.
//! DO NOT EDIT THIS FILE MANUALLY.
//! 
//! To regenerate this file, run:
//! ```bash
//! cargo kit migrate
//! ```
//! 
//! To modify the schema, create a new migration:
//! ```bash
//! cargo kit make:migration add_field_to_users --model User
//! ```
//! 
//! Then edit the migration file and run:
//! ```bash
//! cargo kit migrate
//! ```

use crate::framework::prelude::*;

/// Represents a record in the `users` table.
/// This struct is automatically generated from the database schema.
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct User {
    /// Primary key of the record
    #[sqlx(default)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
