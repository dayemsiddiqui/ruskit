use askama::Template;
use askama_axum::Response;
use crate::framework::views::{Metadata, TemplateExt, HasMetadata};
use axum::response::Html;


/// About page template with custom fields
#[derive(Template, Default)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub first_name: String,
    pub last_name: String,
}

/// Landing page template
#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingTemplate;


/// Renders the about page with team member information
pub async fn about() -> Response {
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

/// Renders the landing page
pub async fn landing() -> Html<String> {
    let template = LandingTemplate;
    Html(template.render().unwrap())
} 