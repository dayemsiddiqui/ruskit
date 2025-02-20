# Entities

Entities in Ruskit represent the data structures of your application. They are the foundation of your models and define the shape of your database tables.

## Entity Structure

An entity is a Rust struct that represents a database table. It defines:
- The fields and their types
- Validation rules through derive macros
- Serialization/deserialization behavior

## Generating Entities

Entities are automatically generated when you create a model:

```bash
cargo kit make:model Post
```

This creates both the entity and its corresponding model file.

## Entity Definition

A basic entity looks like this:

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rustavel_derive::GenerateValidationFields;
use crate::framework::database::model::{Field, ModelValidation};
use validator::ValidationError;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct User {
    #[sqlx(default)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}
```

## Derive Macros

Entities use several important derive macros:

1. `#[derive(Debug, Serialize, Deserialize)]`
   - Enables debugging and JSON serialization/deserialization
   - Required for API responses and database operations

2. `#[derive(FromRow)]`
   - Allows SQLx to map database rows to your entity
   - Automatically converts database types to Rust types

3. `#[derive(GenerateValidationFields)]`
   - Generates validation field definitions
   - Creates the necessary trait implementations for validation
   - Works in conjunction with the `ValidationRules` trait in your model

## Field Attributes

### SQLx Attributes

- `#[sqlx(default)]`: Use default value if field is NULL
- `#[sqlx(rename = "column_name")]`: Map to different column name
- `#[sqlx(type_name = "custom_type")]`: Specify custom SQL type

### Validation Attributes

Validation is handled at the model level through the `ValidationRules` trait. The entity's `GenerateValidationFields` derive macro generates the necessary field definitions.

## Best Practices

1. **Field Types**:
   - Use `i64` for IDs and timestamps
   - Use `String` for text fields
   - Use `Option<T>` for nullable fields
   - Use appropriate numeric types (`i32`, `f64`, etc.)

2. **Naming**:
   - Use PascalCase for entity names (`User`, not `user`)
   - Use singular form (`Post`, not `Posts`)
   - Match database column names (or use `rename` attribute)

3. **Organization**:
   - Keep entities in `src/app/entities/`
   - One entity per file
   - Export through `mod.rs`

4. **Documentation**:
   - Document complex fields
   - Explain validation requirements
   - Note relationships with other entities

## Example Entities

### Basic Entity

```rust
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Post {
    #[sqlx(default)]
    pub id: i64,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
```

### Entity with Relationships

```rust
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Comment {
    #[sqlx(default)]
    pub id: i64,
    pub post_id: i64,  // Foreign key to posts table
    pub user_id: i64,  // Foreign key to users table
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}
```

### Entity with Optional Fields

```rust
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Profile {
    #[sqlx(default)]
    pub id: i64,
    pub user_id: i64,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
```

## Common Patterns

### Timestamps

Always include `created_at` and `updated_at` fields:

```rust
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct User {
    // ... other fields ...
    pub created_at: i64,
    pub updated_at: i64,
}
```

### Foreign Keys

Use descriptive names for foreign keys:

```rust
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Post {
    // ... other fields ...
    pub author_id: i64,  // Better than just user_id
    pub category_id: Option<i64>,  // Optional relationship
}
```

### Custom Types

Use type aliases for clarity:

```rust
pub type Timestamp = i64;
pub type Money = i64;  // Stored in cents

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Order {
    #[sqlx(default)]
    pub id: i64,
    pub amount: Money,
    pub processed_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
``` 