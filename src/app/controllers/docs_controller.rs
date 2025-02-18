use askama::Template;
use askama_axum::Response;
use axum::extract::Path;
use pulldown_cmark::{Parser, Options, html};
use std::fs;
use crate::framework::views::{TemplateExt, HasMetadata, Metadata};

#[derive(Template)]
#[template(path = "docs.html")]
pub struct DocsTemplate {
    pub content: String,
    pub title: String,
    pub description: String,
    pub current_page: String,
    pub sections: Vec<DocSection>,
}

#[derive(Default)]
pub struct DocSection {
    pub title: String,
    pub items: Vec<DocItem>,
}

#[derive(Default)]
pub struct DocItem {
    pub title: String,
    pub path: String,
    pub is_active: bool,
}

impl Default for DocsTemplate {
    fn default() -> Self {
        Self {
            content: String::new(),
            title: String::new(),
            description: String::new(),
            current_page: String::new(),
            sections: vec![
                DocSection {
                    title: "Getting Started".to_string(),
                    items: vec![
                        DocItem {
                            title: "Introduction".to_string(),
                            path: "/docs/introduction".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Installation".to_string(),
                            path: "/docs/installation".to_string(),
                            is_active: false,
                        },
                    ],
                },
                DocSection {
                    title: "Basics".to_string(),
                    items: vec![
                        DocItem {
                            title: "Routing".to_string(),
                            path: "/docs/routing".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Controllers".to_string(),
                            path: "/docs/controllers".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Views".to_string(),
                            path: "/docs/views".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Models".to_string(),
                            path: "/docs/models".to_string(),
                            is_active: false,
                        },
                    ],
                },
                DocSection {
                    title: "Advanced".to_string(),
                    items: vec![
                        DocItem {
                            title: "Middleware".to_string(),
                            path: "/docs/middleware".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Validation".to_string(),
                            path: "/docs/validation".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "DTOs".to_string(),
                            path: "/docs/dtos".to_string(),
                            is_active: false,
                        },
                    ],
                },
                DocSection {
                    title: "Database".to_string(),
                    items: vec![
                        DocItem {
                            title: "Migrations".to_string(),
                            path: "/docs/migrations".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Factories".to_string(),
                            path: "/docs/factories".to_string(),
                            is_active: false,
                        },
                        DocItem {
                            title: "Seeders".to_string(),
                            path: "/docs/seeders".to_string(),
                            is_active: false,
                        },
                    ],
                },
                DocSection {
                    title: "CLI".to_string(),
                    items: vec![
                        DocItem {
                            title: "Commands".to_string(),
                            path: "/docs/commands".to_string(),
                            is_active: false,
                        },
                    ],
                },
            ],
        }
    }
}

pub struct DocsController;

impl DocsController {
    pub async fn show(Path(page): Path<String>) -> Response {
        let path = format!("docs/{}.md", page);
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => return Self::not_found(),
        };

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let lines: Vec<&str> = content.lines().collect();
        let title = lines.first()
            .map(|line| line.trim_start_matches("# ").to_string())
            .unwrap_or_else(|| "Documentation".to_string());
        
        let description = lines.iter()
            .skip(1)
            .find(|line| !line.is_empty())
            .map(|line| line.to_string())
            .unwrap_or_else(|| "Ruskit framework documentation".to_string());

        let mut template = DocsTemplate::with_metadata(
            Metadata::new(&title)
                .with_description(&description)
        );

        // Set the active page
        for section in &mut template.sections {
            for item in &mut section.items {
                item.is_active = item.path == format!("/docs/{}", page);
            }
        }

        template.content = html_output;
        template.title = title;
        template.description = description;
        template.current_page = page;

        template.into_response()
    }

    pub async fn index() -> Response {
        Self::show(Path("introduction".to_string())).await
    }

    fn not_found() -> Response {
        let mut template = DocsTemplate::with_metadata(
            Metadata::new("Not Found")
                .with_description("Page not found")
        );

        template.content = "<h1>Page Not Found</h1><p>The documentation page you're looking for doesn't exist.</p>".to_string();
        template.title = "Not Found".to_string();
        template.description = "Page not found".to_string();
        template.current_page = "404".to_string();

        template.into_response()
    }
} 