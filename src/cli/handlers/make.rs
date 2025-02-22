use crate::cli::error::CliError;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use console::style;

pub fn make_model(name: &str, with_migration: bool) -> Result<(), CliError> {
    // Create entities directory if it doesn't exist
    let entities_dir = Path::new("src/app/entities");
    if !entities_dir.exists() {
        fs::create_dir_all(entities_dir)?;
    }

    // Create models directory if it doesn't exist
    let models_dir = Path::new("src/app/models");
    if !models_dir.exists() {
        fs::create_dir_all(models_dir)?;
    }

    let model_name = name.to_string();
    let table_name = inflector::string::pluralize::to_plural(&model_name.to_lowercase());
    
    // Create entity file
    let entity_file = entities_dir.join(format!("{}.rs", model_name.to_lowercase()));
    let entity_content = format!(
        r#"use serde::{{Deserialize, Serialize}};
use sqlx::FromRow;
use rustavel_derive::GenerateValidationFields;
use crate::framework::database::model::{{Field, ModelValidation}};
use validator::ValidationError;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct {model_name} {{
    #[sqlx(default)]
    pub id: i64,
    // TODO: Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}}"#
    );

    fs::write(&entity_file, entity_content)?;
    println!("Created entity file: {}", entity_file.display());

    // Create model file
    let model_file = models_dir.join(format!("{}.rs", model_name.to_lowercase()));

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let model_content = format!(
        r#"use crate::framework::prelude::*;
use crate::app::entities::{model_name};

impl {model_name} {{
    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {{
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }}
}}

impl ValidationRules for {model_name} {{
    fn validate_rules(&self) -> Result<(), ValidationError> {{
        // TODO: Add your validation rules here
        Ok(())
    }}
}}

#[async_trait]
impl Model for {model_name} {{
    fn table_name() -> &'static str {{
        "{table_name}"
    }}

    fn id(&self) -> i64 {{
        self.id
    }}

    fn factory_definition() -> Self {{
        let now = chrono::Utc::now().timestamp();
        Self {{
            id: 0,
            // TODO: Add your fake data here using Faker
            // Example: name: Faker.fake(),
            created_at: now,
            updated_at: now,
        }}
    }}

    fn migrations() -> Vec<Migration> {{
        vec![
            Migration::create("{timestamp}_create_{table_name}_table", |schema| {{
                schema.create_table("{table_name}", |table| {{
                    table.id();
                    // TODO: Add your columns here
                    table.timestamps();
                }});
            }})
            .down(|schema| {{
                schema.drop_table("{table_name}");
            }})
        ]
    }}
}}"#
    );

    fs::write(&model_file, model_content)?;
    println!("Created model file: {}", model_file.display());

    // Update entities/mod.rs
    let entities_mod_file = entities_dir.join("mod.rs");
    let mut entities_mod_content = String::new();
    
    if entities_mod_file.exists() {
        entities_mod_content = fs::read_to_string(&entities_mod_file)?;
    }

    let mod_line = format!("mod {};", model_name.to_lowercase());
    if !entities_mod_content.contains(&mod_line) {
        if !entities_mod_content.is_empty() {
            entities_mod_content.push('\n');
        }
        entities_mod_content.push_str(&mod_line);
    }

    let use_line = format!("pub use {}::{};", model_name.to_lowercase(), model_name);
    if !entities_mod_content.contains(&use_line) {
        if !entities_mod_content.is_empty() {
            entities_mod_content.push('\n');
        }
        entities_mod_content.push_str(&use_line);
    }

    fs::write(entities_mod_file, entities_mod_content)?;

    // Update models/mod.rs
    let models_mod_file = models_dir.join("mod.rs");
    let mut models_mod_content = String::new();
    
    if models_mod_file.exists() {
        models_mod_content = fs::read_to_string(&models_mod_file)?;
    }

    let mod_line = format!("mod {};", model_name.to_lowercase());
    if !models_mod_content.contains(&mod_line) {
        if !models_mod_content.is_empty() {
            models_mod_content.push('\n');
        }
        models_mod_content.push_str(&mod_line);
    }

    fs::write(models_mod_file, models_mod_content)?;

    if with_migration {
        println!("Don't forget to run migrations with: cargo kit migrate");
    }

    Ok(())
}

pub fn make_migration(name: &str, model: &str) -> Result<(), CliError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let model_file = format!("src/app/models/{}.rs", model.to_lowercase());
    let path = Path::new(&model_file);
    
    if !path.exists() {
        return Err(CliError::FileNotFoundError(format!("Model file {} does not exist", model_file)));
    }
    
    let mut content = fs::read_to_string(path)?;
    
    if let Some(migrations_start) = content.find("fn migrations() -> Vec<Migration> {") {
        if let Some(vec_start) = content[migrations_start..].find("vec![") {
            let vec_content = &content[migrations_start + vec_start..];
            let mut bracket_count = 0;
            let mut vec_end = 0;
            
            for (i, c) in vec_content.chars().enumerate() {
                match c {
                    '[' => bracket_count += 1,
                    ']' => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            vec_end = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
            if vec_end == 0 {
                return Err(CliError::MigrationError("Could not find proper end of migrations vector".into()));
            }
            
            let last_migration_pos = vec_content[..vec_end]
                .rfind("Migration::create")
                .unwrap_or(vec_end);
            
            let insert_pos = if last_migration_pos < vec_end {
                let remaining = &vec_content[last_migration_pos..vec_end];
                if let Some(paren_end) = remaining.find("}),") {
                    migrations_start + vec_start + last_migration_pos + paren_end + 2
                } else {
                    migrations_start + vec_start + vec_end
                }
            } else {
                migrations_start + vec_start + vec_end
            };
            
            let migration = if content[..insert_pos].trim_end().ends_with(',') {
                format!(
                    r#"
            Migration::create("{timestamp}_{name}", |schema| {{
                // TODO: Add your migration schema changes here
                // Example:
                // schema.create_table("table_name", |table| {{
                //     table.id();
                //     table.text("name").not_null();
                //     table.timestamp_iso_strings();
                // }});
            }})
            .down(|schema| {{
                // TODO: Add your rollback schema changes here
                // Example:
                // schema.drop_table("table_name");
            }})"#
                )
            } else {
                format!(
                    r#",
            Migration::create("{timestamp}_{name}", |schema| {{
                // TODO: Add your migration schema changes here
                // Example:
                // schema.create_table("table_name", |table| {{
                //     table.id();
                //     table.text("name").not_null();
                //     table.timestamp_iso_strings();
                // }});
            }})
            .down(|schema| {{
                // TODO: Add your rollback schema changes here
                // Example:
                // schema.drop_table("table_name");
            }})"#
                )
            };
            
            content.insert_str(insert_pos, &migration);
            fs::write(path, content)?;
            
            println!("Created migration {timestamp}_{name}");
            println!("Please edit {} to add your migration schema changes", model_file);
        }
    }
    
    Ok(())
}

pub fn make_controller(name: &str) -> Result<(), CliError> {
    let controller_dir = Path::new("src/app/controllers");
    if !controller_dir.exists() {
        fs::create_dir_all(controller_dir)?;
    }
    
    let controller_name = if name.ends_with("Controller") {
        name.to_string()
    } else {
        format!("{}Controller", name)
    };
    
    let model_exists = Path::new("src/app/models")
        .join(format!("{}.rs", name.to_lowercase()))
        .exists();
    let dto_exists = Path::new("src/app/dtos")
        .join(format!("{}.rs", name.to_lowercase()))
        .exists();
    
    let file_name = controller_dir.join(format!("{}_controller.rs", name.to_lowercase()));
    
    let mut imports = String::from("use crate::framework::prelude::*;");
    
    if model_exists {
        imports.push_str(&format!("\nuse crate::app::entities::{};", name));
    }
    
    if dto_exists {
        imports.push_str(&format!("\nuse crate::app::dtos::{}::{{Create{}Request, {}Response, {}ListResponse}};", 
            name.to_lowercase(), name, name, name));
    }

    let controller_content = format!(r#"{imports}

/// {} Controller handling all {}-related endpoints
pub struct {} {{}}

impl {} {{"#, 
        name, name.to_lowercase(),
        controller_name, controller_name
    );

    let mut methods = String::new();
    
    if model_exists && dto_exists {
        methods.push_str(&format!(r#"
    /// Returns a list of {}s
    pub async fn index() -> Json<{}ListResponse> {{
        match {}::all().await {{
            Ok(items) => Json({}ListResponse::from(items)),
            Err(e) => panic!("Database error: {{}}", e), // In a real app, use proper error handling
        }}
    }}

    /// Returns details for a specific {}
    pub async fn show(Path(id): Path<i64>) -> Json<Option<{}Response>> {{
        match {}::find(id).await {{
            Ok(Some(item)) => Json(Some({}Response::from(item))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {{}}", e), // In a real app, use proper error handling
        }}
    }}

    /// Creates a new {}
    pub async fn store(Json(payload): Json<Create{}Request>) -> Json<{}Response> {{
        let item: {} = payload.into();
        match {}::create(item).await {{
            Ok(created) => Json({}Response::from(created)),
            Err(e) => panic!("Database error: {{}}", e), // In a real app, use proper error handling
        }}
    }}"#,
            name.to_lowercase(), name, name, name,
            name.to_lowercase(), name, name, name,
            name.to_lowercase(), name, name, name, name, name
        ));
    } else {
        methods.push_str("\n    // TODO: Add your controller methods here\n");
        if !model_exists {
            methods.push_str(&format!("    // Note: Create a model first using: cargo kit make:model {}\n", name));
        }
        if !dto_exists {
            methods.push_str(&format!("    // Note: Create DTOs first using: cargo kit make:dto {}\n", name));
        }
    }

    let controller_content = format!("{}\n{}\n}}", controller_content, methods);

    fs::write(&file_name, controller_content)?;

    let mod_file = controller_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_name = format!("{}_controller", name.to_lowercase());
    let mod_line = format!("pub mod {};\npub use {}::*;\n", mod_name, mod_name);
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    println!("Created controller file: {}", file_name.display());
    
    if !model_exists {
        println!("{}", style(format!("Note: Model '{}' does not exist. Create it with: cargo kit make:model {}", name, name)).yellow());
    }
    if !dto_exists {
        println!("{}", style(format!("Note: DTO '{}' does not exist. Create it with: cargo kit make:dto {}", name, name)).yellow());
    }
    
    Ok(())
}

pub fn make_dto(name: &str) -> Result<(), CliError> {
    let dto_dir = Path::new("src/app/dtos");
    if !dto_dir.exists() {
        fs::create_dir_all(dto_dir)?;
    }
    
    let file_name = dto_dir.join(format!("{}.rs", name.to_lowercase()));
    
    let model_exists = Path::new("src/app/models")
        .join(format!("{}.rs", name.to_lowercase()))
        .exists();
    
    let mut imports = String::from("use crate::framework::prelude::*;");
    
    if model_exists {
        imports.push_str(&format!("\nuse crate::app::entities::{};", name));
    }
    
    imports.push_str("\nuse validator::Validate;");
    
    let dto_content = format!(r#"{imports}

#[derive(Serialize)]
pub struct {name}Response {{
    pub id: i64,
    // Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}}

#[derive(Deserialize, Validate)]
pub struct Create{name}Request {{
    // Add your validation fields here
}}

#[derive(Serialize)]
pub struct {name}ListResponse {{
    pub data: Vec<{name}Response>,
}}

impl From<Vec<{name}>> for {name}ListResponse {{
    fn from(items: Vec<{name}>) -> Self {{
        Self {{
            data: items.into_iter().map({name}Response::from).collect(),
        }}
    }}
}}

impl From<{name}> for {name}Response {{
    fn from(item: {name}) -> Self {{
        Self {{
            id: item.id,
            // Map your fields here
            created_at: item.created_at,
            updated_at: item.updated_at,
        }}
    }}
}}

impl From<Create{name}Request> for {name} {{
    fn from(_req: Create{name}Request) -> Self {{
        use std::time::{{SystemTime, UNIX_EPOCH}};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        Self {{
            id: 0,
            // Map your fields here
            created_at: now,
            updated_at: now,
        }}
    }}
}}"#, name=name);

    fs::write(&file_name, dto_content)?;

    let mod_file = dto_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_line = format!("pub mod {};\n", name.to_lowercase());
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    println!("Created DTO file: {}", file_name.display());
    
    if !model_exists {
        println!("{}", style(format!("Note: Model '{}' does not exist. Create it with: cargo kit make:model {}", name, name)).yellow());
    }
    
    Ok(())
}

pub fn make_page_dto(name: &str) -> Result<(), CliError> {
    let dto_dir = Path::new("src/app/dtos");
    if !dto_dir.exists() {
        fs::create_dir_all(dto_dir)?;
    }
    
    let file_name = dto_dir.join(format!("{}.rs", name.to_lowercase()));
    
    let dto_content = format!(r#"use crate::framework::prelude::*;
use ts_export_derive::auto_ts_export;

#[auto_ts_export]
pub struct {name}Props {{
    pub title: String,
    // TODO: Add your page props here
}}"#, name=name);

    fs::write(&file_name, dto_content)?;

    let mod_file = dto_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_line = format!("pub mod {};\n", name.to_lowercase());
    let use_line = format!("pub use {}::{}Props;\n", name.to_lowercase(), name);
    
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
    }
    if !mod_content.contains(&use_line) {
        mod_content.push_str(&use_line);
    }

    fs::write(mod_file, mod_content)?;
    println!("Created DTO file: {}", file_name.display());
    
    Ok(())
}

pub fn make_page_controller(name: &str) -> Result<(), CliError> {
    let controller_dir = Path::new("src/app/controllers");
    if !controller_dir.exists() {
        fs::create_dir_all(controller_dir)?;
    }
    
    let controller_name = format!("{}Controller", name);
    let file_name = controller_dir.join(format!("{}_controller.rs", name.to_lowercase()));
    
    let controller_content = format!(r#"use crate::framework::prelude::*;
use crate::app::dtos::{}::{name}Props;

pub struct {controller_name};

impl {controller_name} {{
    pub async fn show(inertia: Inertia) -> impl IntoResponse {{
        inertia.render("{name}", {name}Props {{
            title: String::from("{name}"),
        }})
    }}
}}"#, name.to_lowercase(), name=name, controller_name=controller_name);

    fs::write(&file_name, controller_content)?;

    let mod_file = controller_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_name = format!("{}_controller", name.to_lowercase());
    let mod_line = format!("pub mod {};\npub use {}::*;\n", mod_name, mod_name);
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    println!("Created controller file: {}", file_name.display());
    
    Ok(())
}

pub fn make_page_component(name: &str) -> Result<(), CliError> {
    let pages_dir = Path::new("resources/js/pages");
    if !pages_dir.exists() {
        fs::create_dir_all(pages_dir)?;
    }
    
    let file_name = pages_dir.join(format!("{}.tsx", name));
    
    let component_content = format!(r#"import React from 'react';
import {{ Head }} from '@inertiajs/react';
import type {{ {name}Props }} from '../types/generated';

interface Props extends {name}Props {{}}

export default function {name}({{ title }}: Props) {{
    return (
        <>
            <Head title={{title}} />
            <div className="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div className="text-center">
                    <h1 className="text-4xl font-bold text-gray-900 sm:text-5xl">
                        {{title}}
                    </h1>
                    {{/* Add your page content here */}}
                </div>
            </div>
        </>
    );
}}"#, name=name);

    fs::write(&file_name, component_content)?;
    println!("Created React component: {}", file_name.display());
    
    Ok(())
} 