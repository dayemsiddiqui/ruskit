use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cache::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Cache::Key).string().not_null().primary_key())
                    .col(ColumnDef::new(Cache::Value).text().not_null())
                    .col(ColumnDef::new(Cache::Expiration).big_integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cache::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Cache {
    Table,
    Key,
    Value,
    Expiration,
} 