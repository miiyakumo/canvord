use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    Published,
    Unpublished,
    Hidden,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArticleMeta {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub last_update: NaiveDateTime,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArticleDetail {
    #[serde(flatten)]
    pub meta: ArticleMeta,
    pub content_md: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageResult<T> {
    pub total: usize,
    pub current: usize,
    pub size: usize,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateArticleCommand {
    pub category: String,
    pub content_md: String,
    pub description: String,
    pub slug: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateArticleCommand {
    pub id: i64,
    pub category: String,
    pub content_md: String,
    pub description: String,
    pub slug: String,
    pub status: Status,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteArticleCommand {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HideArticleCommand {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishArticleCommand {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublishDraftCommand {
    pub id: i64,
    pub category: String,
    pub content_md: String,
    pub description: String,
    pub slug: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveArticleCommand {
    pub category: String,
    pub content_md: String,
    pub description: String,
    pub slug: String,
    pub title: String,
}