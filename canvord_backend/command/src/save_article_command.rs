use crate::IntoActiveModel;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use entity::article;
use entity::article::Status;
use schemars::JsonSchema;
use sea_orm::Set;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SaveArticleCommand {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub content_md: String,
}

impl IntoActiveModel for SaveArticleCommand {
    fn into_active_model(self, now: NaiveDateTime) -> article::ActiveModel {
        article::ActiveModel {
            title: Set(self.title),
            slug: Set(self.slug),
            description: Set(self.description),
            category: Set(self.category),
            content_md: Set(self.content_md),
            created_at: Set(now),
            last_update: Set(now),
            status: Set(Status::Unpublished),
            ..Default::default()
        }
    }
}