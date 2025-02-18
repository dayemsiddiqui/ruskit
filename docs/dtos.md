# Data Transfer Objects (DTOs)

In Ruskit, Data Transfer Objects (DTOs) are structures that define how data should be sent over the network. They help in validating incoming requests and structuring outgoing responses.

## Overview

DTOs serve several purposes:
- Separate the API contract from internal data structures
- Provide input validation for requests
- Control what data is exposed in responses
- Enable versioning of API responses

## Structure

A typical DTO module contains three main structs:

1. Request DTO - For handling incoming data
2. Response DTO - For sending data back to the client
3. List Response DTO - For sending collections of data

### Example

```rust
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::app::models::User;

// Response DTO
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// Request DTO with validation
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

// List Response DTO
#[derive(Serialize)]
pub struct UserListResponse {
    pub data: Vec<UserResponse>,
}
```

## Creating DTOs

Use the `make:dto` command to generate a new DTO:

```bash
cargo kit make:dto User
```

This will create:
- `src/app/dtos/user.rs` with basic DTO structures
- Update `src/app/dtos/mod.rs` to include the new module

## Request DTOs

Request DTOs handle incoming data and validation:

```rust
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(range(min = 0, max = 150))]
    pub age: Option<i32>,
}
```

### Validation Rules

Common validation attributes:
- `length(min = x, max = y)` - String length constraints
- `range(min = x, max = y)` - Numeric range constraints
- `email` - Email format validation
- `url` - URL format validation
- `contains(pattern = "x")` - String contains pattern
- `regex(path = "REGEX")` - Regular expression matching

## Response DTOs

Response DTOs control what data is sent to clients:

```rust
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    // Note: password is not included in response
    pub created_at: i64,
    pub updated_at: i64,
}
```

### List Responses

For collections of data:

```rust
#[derive(Serialize)]
pub struct UserListResponse {
    pub data: Vec<UserResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}
```

## Model Conversion

DTOs should implement `From` traits for conversion:

```rust
// Convert from Model to Response
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// Convert from Request to Model
impl From<CreateUserRequest> for User {
    fn from(req: CreateUserRequest) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        Self {
            id: 0, // New user, ID will be set by database
            name: req.name,
            email: req.email,
            password_hash: hash_password(&req.password),
            created_at: now,
            updated_at: now,
        }
    }
}
```

## Best Practices

1. **Separation of Concerns**
   - Keep DTOs focused on data transfer
   - Don't include business logic in DTOs
   - Use separate DTOs for different API versions

2. **Validation**
   - Always validate incoming data
   - Use appropriate validation rules
   - Consider optional fields when appropriate

3. **Security**
   - Never expose sensitive data in responses
   - Validate all incoming data
   - Use appropriate serialization attributes

4. **Naming Conventions**
   - Use PascalCase for struct names
   - End request DTOs with `Request`
   - End response DTOs with `Response`
   - Use descriptive names for fields

5. **Documentation**
   - Document all DTO fields
   - Include validation requirements
   - Explain any special formatting

## Usage in Controllers

```rust
use axum::{
    response::Json,
    extract::Path,
};
use crate::app::dtos::user::{CreateUserRequest, UserResponse};

impl UserController {
    pub async fn store(
        Json(payload): Json<CreateUserRequest>
    ) -> Json<UserResponse> {
        // Payload is already validated due to DTO
        let user: User = payload.into();
        let created = User::create(user).await?;
        Json(UserResponse::from(created))
    }
} 