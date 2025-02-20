use serde::{Deserialize, Serialize};
use crate::app::entities::User;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
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
        }
    }
}

impl From<CreateUserRequest> for User {
    fn from(request: CreateUserRequest) -> Self {
        Self {
            id: 0,
            name: request.name,
            email: request.email,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }
} 