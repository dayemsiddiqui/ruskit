use axum_inertia::{InertiaConfig as AxumInertiaConfig, vite};
use serde::Serialize;
use serde_json::Value;

pub trait InertiaProps: Serialize {
    fn into_props(self) -> Value where Self: Sized {
        serde_json::to_value(self).expect("Failed to serialize props")
    }
}

// Blanket implementation for all types that implement Serialize
impl<T: Serialize> InertiaProps for T {}

#[derive(Clone)]
pub struct InertiaConfig {
    version: Option<String>,
    root_view: String,
}

impl Default for InertiaConfig {
    fn default() -> Self {
        Self {
            version: None,
            root_view: "app".to_string(),
        }
    }
}

impl InertiaConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_root_view(mut self, root_view: impl Into<String>) -> Self {
        self.root_view = root_view.into();
        self
    }

    pub fn to_axum_config(self) -> AxumInertiaConfig {
        vite::Development::default()
            .port(3000)
            .main("resources/js/app.jsx")
            .lang("en")
            .title("Ruskit")
            .into_config()
    }
} 