use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TodoItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TodoItem::Text).string().not_null())
                    .col(
                        ColumnDef::new(TodoItem::Done)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TodoItem {
    Table,
    Id,
    Text,
    Done,
}
