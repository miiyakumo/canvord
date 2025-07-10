use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
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