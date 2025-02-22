// Framework imports from prelude
use crate::framework::prelude::*;
// App-specific imports
use crate::app::dtos::user::{CreateUserRequest, UserResponse, UserListResponse};
use crate::app::entities::User;

/// User Controller handling all user-related endpoints
pub struct UserController;

impl UserController {
    /// Returns a list of users
    pub async fn index() -> impl IntoResponse {
        // Return hardcoded users
        let users = vec![
            User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                created_at: "2024-03-21".to_string(),
                updated_at: "2024-03-21".to_string(),
            },
            User {
                id: 2,
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
                created_at: "2024-03-21".to_string(),
                updated_at: "2024-03-21".to_string(),
            },
        ];
        Json(UserListResponse::from(users))
    }

    /// Returns details for a specific user
    pub async fn show(Path(id): Path<i64>) -> impl IntoResponse {
        // Return hardcoded user if id matches
        let user = if id == 1 {
            Some(User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                created_at: "2024-03-21".to_string(),
                updated_at: "2024-03-21".to_string(),
            })
        } else {
            None
        };
        Json(user.map(UserResponse::from))
    }

    /// Creates a new user
    pub async fn store(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
        // Create a new user from the payload
        let user = User {
            id: 3, // Hardcoded new ID
            name: payload.name,
            email: payload.email,
            created_at: "2024-03-21".to_string(),
            updated_at: "2024-03-21".to_string(),
        };
        Json(UserResponse::from(user))
    }

    pub async fn recent() -> impl IntoResponse {
        // Return same hardcoded users as index
        let users = vec![
            User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                created_at: "2024-03-21".to_string(),
                updated_at: "2024-03-21".to_string(),
            },
            User {
                id: 2,
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
                created_at: "2024-03-21".to_string(),
                updated_at: "2024-03-21".to_string(),
            },
        ];
        Json(UserListResponse::from(users))
    }
} 