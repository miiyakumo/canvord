use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // 创建表
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Article::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::Slug).string().not_null())
                    .col(ColumnDef::new(Article::Description).string().not_null())
                    // MEDIUMTEXT 类型，需要用 custom 类型声明
                    .col(ColumnDef::new(Article::Content).custom("MEDIUMTEXT").not_null())
                    .col(ColumnDef::new(Article::Category).string().not_null())
                    .col(ColumnDef::new(Article::LastUpdate).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    // 回滚删除表
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    Title,
    Slug,
    Description,
    Content,
    Category,
    LastUpdate,
}