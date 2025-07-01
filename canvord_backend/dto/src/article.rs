use apistos::ApiComponent;
use chrono::NaiveDateTime;
use entity::article::{ActiveModel, Model, Status};
use schemars::JsonSchema;
use sea_orm::{DbErr, TryIntoModel};

#[derive(Debug, Clone, serde::Serialize, JsonSchema, ApiComponent)]
pub struct ArticleMeta {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub last_update: NaiveDateTime,
    pub status: Status,
}

#[derive(Debug, Clone, serde::Serialize, JsonSchema, ApiComponent)]
pub struct ArticleDetail {
    #[serde(flatten)]
    pub meta: ArticleMeta,
    pub content_md: String,
    pub created_at: NaiveDateTime,
}

impl From<Model> for ArticleMeta {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            title: m.title,
            slug: m.slug,
            description: m.description,
            category: m.category,
            last_update: m.last_update,
            status: m.status,
        }
    }
}

impl From<Model> for ArticleDetail {
    fn from(m: Model) -> Self {
        Self {
            meta: m.clone().into(),
            content_md: m.content_md,
            created_at: m.created_at,
        }
    }
}

impl TryFrom<ActiveModel> for ArticleDetail {
    type Error = DbErr;

    fn try_from(am: ActiveModel) -> Result<Self, Self::Error> {
        let model = am.try_into_model()?;
        Ok(model.into())
    }
}
