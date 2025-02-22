use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Jobs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Jobs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Jobs::Queue).string().not_null())
                    .col(ColumnDef::new(Jobs::Payload).text().not_null())
                    .col(ColumnDef::new(Jobs::Attempts).unsigned().not_null().default(0))
                    .col(ColumnDef::new(Jobs::ReservedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(Jobs::AvailableAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Jobs::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Add index for queue lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_jobs_queue")
                    .table(Jobs::Table)
                    .col(Jobs::Queue)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Jobs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Jobs {
    Table,
    Id,
    Queue,
    Payload,
    Attempts,
    ReservedAt,
    AvailableAt,
    CreatedAt,
}
