use std::collections::HashMap;
use validator::{Validate, ValidationError};
use serde::Deserialize;

pub trait Rules {
    fn rules() -> HashMap<&'static str, Vec<&'static str>>;
}

pub trait Validator {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

#[derive(Debug)]
pub struct ValidationErrors {
    errors: HashMap<String, Vec<String>>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    pub fn add(&mut self, field: &str, message: &str) {
        self.errors
            .entry(field.to_string())
            .or_insert_with(Vec::new)
            .push(message.to_string());
    }

    pub fn errors(&self) -> &HashMap<String, Vec<String>> {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

// Validation macros similar to Laravel
#[macro_export]
macro_rules! validate {
    ($data:expr) => {
        $data.validate()
    };
}

// Example of a validation rule struct
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

// Example implementation of Rules trait
impl Rules for LoginRequest {
    fn rules() -> HashMap<&'static str, Vec<&'static str>> {
        let mut rules = HashMap::new();
        rules.insert("email", vec!["required", "email"]);
        rules.insert("password", vec!["required", "min:6"]);
        rules
    }
}

// Built-in validation rules
pub mod rules {
    use super::*;
    use regex::Regex;

    pub fn required(value: &str) -> Result<(), ValidationError> {
        if value.trim().is_empty() {
            return Err(ValidationError::new("required"));
        }
        Ok(())
    }

    pub fn email(value: &str) -> Result<(), ValidationError> {
        let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
        if !re.is_match(value) {
            return Err(ValidationError::new("email"));
        }
        Ok(())
    }

    pub fn min(value: &str, min: usize) -> Result<(), ValidationError> {
        if value.len() < min {
            return Err(ValidationError::new("min"));
        }
        Ok(())
    }

    pub fn max(value: &str, max: usize) -> Result<(), ValidationError> {
        if value.len() > max {
            return Err(ValidationError::new("max"));
        }
        Ok(())
    }
} 