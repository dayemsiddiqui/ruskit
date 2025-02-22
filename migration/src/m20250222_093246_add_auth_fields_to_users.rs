use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add password column
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Password).string().not_null().default(""))
                    .to_owned(),
            )
            .await?;

        // Add role column
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Role).string().not_null().default("user"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop role column
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Role)
                    .to_owned(),
            )
            .await?;

        // Drop password column
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Password)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Password,
    Role,
}
