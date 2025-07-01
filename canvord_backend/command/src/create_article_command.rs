use crate::IntoActiveModel;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use entity::article;
use entity::article::Status;
use schemars::JsonSchema;
use sea_orm::Set;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct CreateArticleCommand {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub content_md: String,
}

impl IntoActiveModel for CreateArticleCommand {
    fn into_active_model(self, now: NaiveDateTime) -> article::ActiveModel {
        article::ActiveModel {
            title: Set(self.title),
            slug: Set(self.slug),
            description: Set(self.description),
            content_md: Set(self.content_md),
            category: Set(self.category),
            created_at: Set(now),
            last_update: Set(now),
            status: Set(Status::Published),
            ..Default::default()
        }
    }
}