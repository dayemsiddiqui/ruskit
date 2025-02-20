# Validation

Ruskit provides a powerful validation system that combines compile-time validation field generation with runtime validation rules.

## Overview

The validation system in Ruskit consists of two main components:

1. **Entity-Level Validation Fields**
   - Generated using the `GenerateValidationFields` derive macro
   - Defines which fields can be validated
   - Provides type-safe field access

2. **Model-Level Validation Rules**
   - Implemented through the `ValidationRules` trait
   - Defines specific validation rules for each field
   - Executed during model operations

## Entity Validation Setup

First, add the `GenerateValidationFields` derive macro to your entity:

```rust
use rustavel_derive::GenerateValidationFields;

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

## Model Validation Rules

Then implement the `ValidationRules` trait in your model:

```rust
use crate::framework::database::model::{Rules, Validate, ValidationRules};

impl ValidationRules for User {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        self.name.validate(Rules::new().required().min(3).max(255))?;
        self.email.validate(Rules::new().required().email())?;
        Ok(())
    }
}
```

## Available Rules

Ruskit provides several built-in validation rules:

```rust
Rules::new()
    .required()           // Field must not be empty
    .email()             // Must be valid email format
    .min(3)              // Minimum length
    .max(255)            // Maximum length
    .regex("[0-9]+")     // Must match regex pattern
    .in_array(&["a", "b"]) // Must be one of these values
```

## Custom Validation Rules

You can create custom validation rules by implementing the `Rule` trait:

```rust
pub struct PhoneNumber;

impl Rule for PhoneNumber {
    fn validate(&self, field: &str, value: &str) -> Result<(), ValidationError> {
        if !value.chars().all(|c| c.is_numeric() || c == '+' || c == '-') {
            return Err(ValidationError::new("invalid phone number"));
        }
        Ok(())
    }
}

// Use in validation rules
impl ValidationRules for Contact {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        self.phone.validate(Rules::new().required().add_rule(PhoneNumber))?;
        Ok(())
    }
}
```

## Validation in Controllers

Validation is automatically applied when creating or updating models:

```rust
impl UserController {
    pub async fn store(Json(payload): Json<CreateUserRequest>) -> Result<Json<UserResponse>, ValidationError> {
        let user: User = payload.into();
        let validated_user = User::create_validated(user).await?;
        Ok(Json(UserResponse::from(validated_user)))
    }
}
```

## DTO Validation

You can also use validation in your DTOs:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), ValidationError> {
        validator::Validate::validate(self)
    }
}
```

## Best Practices

1. **Entity Validation**:
   - Use `GenerateValidationFields` for all entities
   - Keep validation fields in sync with database schema

2. **Model Rules**:
   - Implement `ValidationRules` for all models
   - Keep validation rules close to business logic
   - Use descriptive error messages

3. **Custom Rules**:
   - Create reusable validation rules
   - Document rule requirements
   - Test edge cases

4. **Error Handling**:
   - Return appropriate error responses
   - Include field-specific error messages
   - Log validation failures for debugging 