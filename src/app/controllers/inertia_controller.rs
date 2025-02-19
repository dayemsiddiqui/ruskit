use axum::response::IntoResponse;
use axum_inertia::Inertia;
use serde_json::json;
use crate::app::dtos::about::AboutPageProps;
pub struct InertiaController;

impl InertiaController {
    pub async fn about(inertia: Inertia) -> impl IntoResponse {
        let props = AboutPageProps {
            title: String::from("About"),
            description: String::from("Learn more about Ruskit"),
            tech_stack: vec![String::from("Rust"), String::from("React"),
             String::from("TypeScript"),
             String::from("Tailwind CSS")],
        };

        inertia.render(
            "About",
            json!(props)
        )
    }
} 