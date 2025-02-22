pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240320_000001_create_cache_table;
mod m20250222_022856_create_posts_table;
mod m20250222_030920_create_comments_table;
mod m20250222_093246_add_auth_fields_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240320_000001_create_cache_table::Migration),
            Box::new(m20250222_022856_create_posts_table::Migration),
            Box::new(m20250222_030920_create_comments_table::Migration),
            Box::new(m20250222_093246_add_auth_fields_to_users::Migration),
        ]
    }
}
