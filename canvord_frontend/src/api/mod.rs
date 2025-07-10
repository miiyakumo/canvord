// src/api/mod.rs
pub mod auth;

use gloo_net::http::{Request, RequestBuilder};
use crate::model::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::auth::load_token;

const API_BASE: &str = "http://localhost:8000/articles";

// 加 JWT Header（如果存在）
fn with_auth(mut req: RequestBuilder) -> RequestBuilder {
    if let Some(token) = load_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    req
}

async fn post_json<T: Serialize, R: DeserializeOwned>(url: &str, body: &T) -> Result<AppResponse<R>, String> {
    with_auth(Request::post(url))
        .header("Content-Type", "application/json")
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

async fn get_json<R: DeserializeOwned>(url: &str) -> Result<AppResponse<R>, String> {
    with_auth(Request::get(url))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

async fn put_json<T: Serialize, R: DeserializeOwned>(url: &str, body: &T) -> Result<AppResponse<R>, String> {
    with_auth(Request::put(url))
        .header("Content-Type", "application/json")
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

async fn delete_json<T: Serialize, R: DeserializeOwned>(url: &str, body: &T) -> Result<AppResponse<R>, String> {
    with_auth(Request::delete(url))
        .header("Content-Type", "application/json")
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_article(cmd: &CreateArticleCommand) -> Result<AppResponse<ArticleDetail>, String> {
    post_json(&format!("{API_BASE}/create"), cmd).await
}

pub async fn update_article(cmd: &UpdateArticleCommand) -> Result<AppResponse<ArticleDetail>, String> {
    put_json(&format!("{API_BASE}/update"), cmd).await
}

pub async fn delete_article(cmd: &DeleteArticleCommand) -> Result<AppResponse<u64>, String> {
    delete_json(&format!("{API_BASE}/delete"), cmd).await
}

pub async fn publish_article(cmd: &PublishArticleCommand) -> Result<AppResponse<ArticleDetail>, String> {
    put_json(&format!("{API_BASE}/publish"), cmd).await
}

pub async fn hide_article(cmd: &HideArticleCommand) -> Result<AppResponse<ArticleDetail>, String> {
    put_json(&format!("{API_BASE}/hide"), cmd).await
}

pub async fn publish_draft(cmd: &PublishDraftCommand) -> Result<AppResponse<ArticleDetail>, String> {
    put_json(&format!("{API_BASE}/publish-draft"), cmd).await
}

pub async fn save_draft(cmd: &SaveArticleCommand) -> Result<AppResponse<ArticleDetail>, String> {
    put_json(&format!("{API_BASE}/save-draft"), cmd).await
}

pub async fn get_article_by_id(id: i64) -> Result<AppResponse<ArticleDetail>, String> {
    get_json(&format!("{API_BASE}/id/{}", id)).await
}

pub async fn get_article_by_slug(slug: &str) -> Result<AppResponse<ArticleDetail>, String> {
    get_json(&format!("{API_BASE}/slug/{}", slug)).await
}

pub async fn get_article_by_title(title: &str) -> Result<AppResponse<Vec<ArticleMeta>>, String> {
    get_json(&format!("{API_BASE}/title/{}", title)).await
}

pub async fn get_article_page(page: u64, per: u64, status: Option<Status>) -> Result<AppResponse<PageResult<ArticleMeta>>, String> {
    let mut url = format!("{API_BASE}/page?page={}&per={}", page, per);
    if let Some(s) = status {
        url.push_str(&format!("&status={}", match s {
            Status::Published => "Published",
            Status::Unpublished => "Unpublished",
            Status::Hidden => "Hidden",
        }));
    }
    get_json(&url).await
}
