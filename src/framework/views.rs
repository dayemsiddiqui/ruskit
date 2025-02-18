use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    http::StatusCode,
};
use askama_axum::Response;
use std::{sync::OnceLock, cell::RefCell, thread_local};

static GLOBAL_METADATA: OnceLock<Metadata> = OnceLock::new();

thread_local! {
    static LOCAL_METADATA: RefCell<Option<Metadata>> = RefCell::new(None);
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub author: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: "Ruskit".to_string(),
            description: None,
            keywords: None,
            author: None,
            og_title: None,
            og_description: None,
            og_image: None,
        }
    }
}

impl Metadata {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn with_og_title(mut self, title: impl Into<String>) -> Self {
        self.og_title = Some(title.into());
        self
    }

    pub fn with_og_description(mut self, description: impl Into<String>) -> Self {
        self.og_description = Some(description.into());
        self
    }

    pub fn with_og_image(mut self, image: impl Into<String>) -> Self {
        self.og_image = Some(image.into());
        self
    }
}

pub fn set_global_metadata(metadata: Metadata) {
    let _ = GLOBAL_METADATA.set(metadata);
}

pub fn get_global_metadata() -> &'static Metadata {
    GLOBAL_METADATA.get_or_init(Metadata::default)
}

// Trait for templates with metadata
pub trait HasMetadata: Default {
    fn metadata(&self) -> &'static Metadata {
        LOCAL_METADATA.with(|local| {
            if let Some(metadata) = &*local.borrow() {
                // SAFETY: The metadata lives for the duration of the request
                // and is cleared after the response is sent
                unsafe { std::mem::transmute(metadata) }
            } else {
                get_global_metadata()
            }
        })
    }

    fn with_metadata(metadata: Metadata) -> Self where Self: Sized {
        LOCAL_METADATA.with(|local| {
            *local.borrow_mut() = Some(metadata);
        });
        Self::default()
    }
}

// Implement HasMetadata for all Template types that are unit structs
impl<T: Template> HasMetadata for T where T: Default {}

// Extension trait for Template types
pub trait TemplateExt: Template + HasMetadata + Sized {
    fn into_response(self) -> Response {
        let response = match self.render() {
            Ok(html) => Html(html).into_response().into(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response().into(),
        };
        // Clear the local metadata after the response is generated
        LOCAL_METADATA.with(|local| {
            *local.borrow_mut() = None;
        });
        response
    }
}

// Implement TemplateExt for all Template types
impl<T: Template + HasMetadata> TemplateExt for T {} 