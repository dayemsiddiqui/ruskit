use serde::{Serialize, Deserialize};
use crate::app::models::User;
use validator::Validate;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
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
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<CreateUserRequest> for User {
    fn from(req: CreateUserRequest) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        Self {
            id: 0,
            name: req.name,
            email: req.email,
            created_at: now,
            updated_at: now,
        }
    }
} 