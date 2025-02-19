use axum_inertia::{InertiaConfig as AxumInertiaConfig, vite};

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