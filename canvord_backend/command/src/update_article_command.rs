use crate::IntoActiveModel;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use entity::article;
use entity::article::Status;
use schemars::JsonSchema;
use sea_orm::Set;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateArticleCommand {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub content_md: String,
    pub status: Status,
}

impl IntoActiveModel for UpdateArticleCommand {
    fn into_active_model(self, now: NaiveDateTime) -> article::ActiveModel {
        article::ActiveModel {
            id: Set(self.id),
            title: Set(self.title),
            slug: Set(self.slug),
            description: Set(self.description),
            category: Set(self.category),
            content_md: Set(self.content_md),
            last_update: Set(now),
            status: Set(self.status),
            ..Default::default()
        }
    }
}