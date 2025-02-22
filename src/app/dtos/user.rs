use crate::app::entities::user::Model as User;
use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub data: Vec<UserResponse>,
}

impl From<Vec<User>> for UserListResponse {
    fn from(users: Vec<User>) -> Self {
        Self {
            data: users.into_iter().map(UserResponse::from).collect(),
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            created_at: Some(user.created_at),
            updated_at: Some(user.updated_at),
        }
    }
}

impl From<Option<User>> for UserResponse {
    fn from(user: Option<User>) -> Self {
        match user {
            Some(user) => Self::from(user),
            None => Self {
                id: 0,
                name: String::new(),
                email: String::new(),
                role: String::from("user"),
                created_at: None,
                updated_at: None,
            }
        }
    }
}

impl From<CreateUserRequest> for User {
    fn from(request: CreateUserRequest) -> Self {
        let now = chrono::Utc::now().timestamp().to_string();
        Self {
            id: 0,
            name: request.name,
            email: request.email,
            password: String::new(), // This will be set by the controller
            role: "user".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
} 