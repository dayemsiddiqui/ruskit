use axum::{
    Router,
    routing::{get, post},
    response::Json,
    extract::Path,
};
use askama::Template;
use askama_axum::Response;
use serde_json::{json, Value};

use crate::framework::{
    middleware::{
        WithMiddleware,
        presets::{Cors, TrimStrings},
    },
    views::{Metadata, TemplateExt, HasMetadata},
};
use crate::bootstrap::app::bootstrap;

/// Home page template
#[derive(Template, Default)]
#[template(path = "home.html")]
pub struct HomeTemplate;

/// About page template with custom fields
#[derive(Template, Default)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub first_name: String,
    pub last_name: String,
}

// Example of how to use metadata in a route handler
async fn home() -> Response {
    HomeTemplate::default().into_response()
}   

// Example of overriding metadata for a specific route
async fn about() -> Response {
    let mut about_template = AboutTemplate::with_metadata(
        Metadata::new("About Us")
            .with_description("Learn more about our team")
            .with_og_title("About Us")
            .with_og_description("Meet John Doe, a key member of our team")
    );
    
    about_template.first_name = "John".to_string();
    about_template.last_name = "Doe".to_string();
    
    about_template.into_response()
}

// Route handlers
async fn index() -> Json<Value> {
    Json(json!({ "message": "Welcome to Ruskit!" }))
}

async fn users_index() -> Json<Value> {
    Json(json!({ "message": "List of users" }))
}

async fn users_show(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "message": "Show user details",
        "id": id
    }))
}

async fn users_store() -> Json<Value> {
    Json(json!({ "message": "Create new user" }))
}

// Define routes with middleware
pub async fn routes() -> Router {
    bootstrap().await;
    
    let router = Router::new()
        .route(
            "/", 
            get(home).middleware(Cors::new("http://specific.example.com"))
        )
        .route("/about", get(about));

    let api_router = Router::new()
        .route("/", get(index))
        .route(
            "/users", 
            get(users_index)
                .middleware(TrimStrings::new())
                .middleware(Cors::new("http://users.example.com"))
        )
        .route("/users/{id}", get(users_show))
        .route(
            "/users", 
            post(users_store).middleware(TrimStrings::new())
        );

    router.nest("/api", api_router)
} 