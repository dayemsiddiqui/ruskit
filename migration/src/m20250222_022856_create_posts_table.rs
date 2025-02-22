use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Content).text().not_null())
                    .col(ColumnDef::new(Post::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Post::CreatedAt).timestamp_with_time_zone().not_null().default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())))
                    .col(ColumnDef::new(Post::UpdatedAt).timestamp_with_time_zone().not_null().default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Content,
    Slug,
    CreatedAt,
    UpdatedAt,
}
