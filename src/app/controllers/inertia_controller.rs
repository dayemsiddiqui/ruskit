use axum::response::IntoResponse;
use axum_inertia::Inertia;
use serde_json::json;

pub struct InertiaController;

impl InertiaController {
    pub async fn home(inertia: Inertia) -> impl IntoResponse {
        inertia.render(
            "Home",
            json!({
                "title": "Welcome",
                "description": "Welcome to Ruskit with Inertia"
            })
        )
    }

    pub async fn about(inertia: Inertia) -> impl IntoResponse {
        inertia.render(
            "About",
            json!({
                "title": "About",
                "description": "Learn more about Ruskit"
            })
        )
    }
} 