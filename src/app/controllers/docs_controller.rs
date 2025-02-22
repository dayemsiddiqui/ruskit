use askama::Template;
use askama_axum::{Response, IntoResponse};
use axum::extract::Path;

#[derive(Template)]
#[template(path = "docs/commands.html")]
pub struct CommandsTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/routing.html")]
pub struct RoutingTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/controllers.html")]
pub struct ControllersTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/models.html")]
pub struct ModelsTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/views.html")]
pub struct ViewsTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/validation.html")]
pub struct ValidationTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/dtos.html")]
pub struct DtosTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/entities.html")]
pub struct EntitiesTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/middleware.html")]
pub struct MiddlewareTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/extractors.html")]
pub struct ExtractorsTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/frontend.html")]
pub struct FrontendTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/queues.html")]
pub struct QueuesTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
}

#[derive(Template)]
#[template(path = "docs/test.html")]
pub struct TestTemplate {
    pub content: String,
    pub sections: Vec<DocSection>,
    pub current_page: String,
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

pub struct DocsController;

impl DocsController {
    fn get_sections_with_active(page: &str) -> Vec<DocSection> {
        let mut sections = Vec::new();
        let default_sections = vec![
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
                ],
            },
            DocSection {
                title: "Database & Models".to_string(),
                items: vec![
                    DocItem {
                        title: "Entities".to_string(),
                        path: "/docs/entities".to_string(),
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
                title: "Frontend".to_string(),
                items: vec![
                    DocItem {
                        title: "Frontend Development".to_string(),
                        path: "/docs/frontend".to_string(),
                        is_active: false,
                    },
                ],
            },
            DocSection {
                title: "Background Tasks".to_string(),
                items: vec![
                    DocItem {
                        title: "Queue System".to_string(),
                        path: "/docs/queues".to_string(),
                        is_active: false,
                    },
                    DocItem {
                        title: "Task Scheduling".to_string(),
                        path: "/docs/schedule".to_string(),
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
                    DocItem {
                        title: "Extractors".to_string(),
                        path: "/docs/extractors".to_string(),
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
        ];

        for section in default_sections {
            let mut new_section = DocSection {
                title: section.title,
                items: Vec::new(),
            };
            for item in section.items {
                let path_page = item.path.strip_prefix("/docs/").unwrap_or(&item.path);
                new_section.items.push(DocItem {
                    title: item.title,
                    path: item.path.clone(),
                    is_active: path_page == page,
                });
            }
            sections.push(new_section);
        }
        sections
    }

    pub async fn show(Path(page): Path<String>) -> Response {
        let sections = Self::get_sections_with_active(&page);
        
        match page.as_str() {
            "commands" => CommandsTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "routing" => RoutingTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "controllers" => ControllersTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "models" => ModelsTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "views" => ViewsTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "validation" => ValidationTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "dtos" => DtosTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "entities" => EntitiesTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "middleware" => MiddlewareTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "extractors" => ExtractorsTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "frontend" => FrontendTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            "queues" => QueuesTemplate {
                content: String::new(),
                sections,
                current_page: page.clone(),
            }.into_response(),
            _ => CommandsTemplate {
                content: String::new(),
                sections,
                current_page: page,
            }.into_response(),
        }
    }

    pub async fn index() -> Response {
        let sections = Self::get_sections_with_active("introduction");
        CommandsTemplate {
            content: String::new(),
            sections,
            current_page: "introduction".to_string(),
        }.into_response()
    }
} 