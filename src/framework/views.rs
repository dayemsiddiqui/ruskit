use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    http::StatusCode,
};

pub trait View {
    fn render(&self) -> Result<Html<String>, StatusCode>;
}

impl<T: Template> View for T {
    fn render(&self) -> Result<Html<String>, StatusCode> {
        self.render()
            .map(Html)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// Wrapper type to implement IntoResponse
pub struct ViewResponse<T>(pub T);

impl<T: View> IntoResponse for ViewResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => html.into_response(),
            Err(status) => status.into_response(),
        }
    }
} 