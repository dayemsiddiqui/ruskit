use crate::app::entities::user::Model as User;
use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
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
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<CreateUserRequest> for User {
    fn from(request: CreateUserRequest) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: 0,
            name: request.name,
            email: request.email,
            created_at: now.to_string(),
            updated_at: now.to_string(),
        }
    }
} 