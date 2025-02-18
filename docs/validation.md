# Validation in Ruskit

Ruskit provides a powerful validation system similar to Laravel, allowing you to validate incoming request data, form inputs, or any other data in your application.

## Basic Usage

Here's a basic example of how to use validation in your request handlers:

```rust
use ruskit::framework::validation::{Rules, Validate};
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 2))]
    pub name: String,
}

impl Rules for CreateUserRequest {
    fn rules() -> HashMap<&'static str, Vec<&'static str>> {
        let mut rules = HashMap::new();
        rules.insert("email", vec!["required", "email"]);
        rules.insert("password", vec!["required", "min:6"]);
        rules.insert("name", vec!["required", "min:2"]);
        rules
    }
}

// In your route handler:
async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl Response, ValidationErrors> {
    // Validate the request
    validate!(payload)?;
    
    // If validation passes, continue with your logic
    // ...
}
```

## Available Validation Rules

Ruskit provides several built-in validation rules:

- `required`: The field must be present and not empty
- `email`: The field must be a valid email address
- `min:value`: The field must be at least the specified length
- `max:value`: The field must not exceed the specified length

## Custom Validation Rules

You can create custom validation rules by implementing custom validators:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CustomValidationExample {
    #[validate(custom = "validate_even")]
    number: i32,
}

fn validate_even(value: &i32) -> Result<(), ValidationError> {
    if value % 2 == 0 {
        Ok(())
    } else {
        Err(ValidationError::new("must be even"))
    }
}
```

## Handling Validation Errors

When validation fails, it returns a `ValidationErrors` struct that contains all validation errors:

```rust
match validate!(request) {
    Ok(_) => {
        // Validation passed
    }
    Err(errors) => {
        // Access validation errors
        for (field, messages) in errors.errors() {
            println!("Field '{}' has errors: {:?}", field, messages);
        }
    }
}
```

## Form Request Validation

For more complex validation scenarios, you can create dedicated Form Request types:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

impl Rules for LoginRequest {
    fn rules() -> HashMap<&'static str, Vec<&'static str>> {
        let mut rules = HashMap::new();
        rules.insert("email", vec!["required", "email"]);
        rules.insert("password", vec!["required", "min:6"]);
        rules
    }
}
```

This validation system provides a familiar Laravel-like experience while leveraging Rust's type system and compile-time checks. 