use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TextItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TextItem::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TextItem::ItemId)
                            .big_integer()
                            .null()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(TextItem::Content)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TextItem::UserIdentityUuid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TextItem::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .index(
                        Index::create()
                            .name("idx_itemId")
                            .table(TextItem::Table)
                            .col(TextItem::ItemId),
                    )
                    .index(
                        Index::create()
                            .name("idx_userIdentityUuid")
                            .table(TextItem::Table)
                            .col(TextItem::UserIdentityUuid),
                    )
                    .index(
                        Index::create()
                            .name("idx_isDeleted")
                            .table(TextItem::Table)
                            .col(TextItem::IsDeleted),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TextItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TextItem {
    Table,
    Id,
    ItemId,
    Content,
    UserIdentityUuid,
    IsDeleted,
}
