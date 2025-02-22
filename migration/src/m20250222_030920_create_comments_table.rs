use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comments::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Comments::Title).string().not_null())
                    .col(ColumnDef::new(Comments::Text).text().not_null())
                    .col(ColumnDef::new(Comments::PostId).integer().not_null())
                    .col(ColumnDef::new(Comments::UserId).integer().not_null())
                    .col(ColumnDef::new(Comments::CreatedAt).timestamp_with_time_zone().not_null().default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())))
                    .col(ColumnDef::new(Comments::UpdatedAt).timestamp_with_time_zone().not_null().default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    Title,
    Text,
    PostId,
    UserId,
    CreatedAt,
    UpdatedAt,
}
