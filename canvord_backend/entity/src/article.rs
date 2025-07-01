use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::{DeleteMany, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content_md: String,
    pub category: String,
    pub created_at: DateTime,
    pub last_update: DateTime,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, JsonSchema)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "camelCase")]
pub enum Status {
    Published,
    Unpublished,
    Hidden,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn from_model(data: &Model, status: Status, now: DateTime) -> Self {
        Self {
            title: Set(data.title.to_owned()),
            slug: Set(data.slug.to_owned()),
            description: Set(data.description.to_owned()),
            content_md: Set(data.content_md.to_owned()),
            category: Set(data.category.to_owned()),
            created_at: Set(now),
            last_update: Set(now),
            status: Set(status),
            ..Default::default()
        }
    }
}

impl Entity {
    pub fn find_by_id(id: i64) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_slug(slug: &str) -> Select<Entity> {
        Self::find().filter(Column::Slug.eq(slug))
    }

    pub fn list_by_title(title: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.like(title))
    }

    pub fn list_by_status(status: Status) -> Select<Entity> {
        Self::find().filter(Column::Status.eq(status))
    }

    pub fn delete_by_id(id: i64) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}