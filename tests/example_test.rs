use ruskit::framework::testing::{test, HttpAssertions, DatabaseAssertions, Assertions};
use ruskit::app::models::User;
use ruskit::framework::database::ModelFactory;
use axum::{
    Router,
    routing::{post, get},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;

#[tokio::test]
async fn test_user_registration() {
    // Setup the application router
    let app = Router::new()
        .route("/register", post(register_handler));

    // Run the test
    test(app)
        .post("/register", Some(json!({
            "name": "John Doe",
            "email": "john@example.com",
            "password": "password123",
            "password_confirmation": "password123"
        })))
        .await
        .assert_created()
        .assert_json(json!({
            "message": "User registered successfully"
        }))
        .assert_database_has("users", json!({
            "email": "john@example.com"
        }));
}

#[tokio::test]
async fn test_user_login() {
    // Setup
    let app = Router::new()
        .route("/login", post(login_handler));
        
    // Create a test user
    let user = User::factory()
        .email("test@example.com")
        .password("password123")
        .create()
        .await;
        
    // Test valid login
    test(app.clone())
        .post("/login", Some(json!({
            "email": "test@example.com",
            "password": "password123"
        })))
        .await
        .assert_ok()
        .assert_json_has("token");
        
    // Test invalid login
    test(app)
        .post("/login", Some(json!({
            "email": "test@example.com",
            "password": "wrong_password"
        })))
        .await
        .assert_status(StatusCode::UNAUTHORIZED)
        .assert_json(json!({
            "message": "Invalid credentials"
        }));
}

#[tokio::test]
async fn test_protected_route() {
    // Setup
    let app = Router::new()
        .route("/profile", get(profile_handler));
        
    // Create a test user
    let user = User::factory().create().await;
    
    // Test unauthorized access
    test(app.clone())
        .get("/profile")
        .await
        .assert_status(StatusCode::UNAUTHORIZED);
        
    // Test authorized access
    test(app)
        .acting_as(user)
        .get("/profile")
        .await
        .assert_ok()
        .assert_json_has("user");
}

#[tokio::test]
async fn test_validation() {
    let app = Router::new()
        .route("/users", post(create_user_handler));
        
    // Test validation errors
    test(app)
        .post("/users", Some(json!({
            "name": "",  // Empty name
            "email": "invalid-email"  // Invalid email
        })))
        .await
        .assert_has_errors(&["name", "email"]);
}

// Example handlers (these would be in your actual application code)
async fn register_handler() -> impl IntoResponse {
    // Implementation
    todo!()
}

async fn login_handler() -> impl IntoResponse {
    // Implementation
    todo!()
}

async fn profile_handler() -> impl IntoResponse {
    // Implementation
    todo!()
}

async fn create_user_handler() -> impl IntoResponse {
    // Implementation
    todo!()
} 