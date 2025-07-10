// src/api/mod.rs
use gloo_net::http::Request;
use crate::model::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

const API_BASE: &str = "http://localhost:8000/visitor";

async fn post_json<T: Serialize, R: DeserializeOwned>(url: &str, body: &T) -> Result<AppResponse<R>, String> {
    Request::post(url)
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
    Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

async fn put_json<T: Serialize, R: DeserializeOwned>(url: &str, body: &T) -> Result<AppResponse<R>, String> {
    Request::put(url)
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
    Request::delete(url)
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

pub async fn get_article_by_slug(slug: &str) -> Result<AppResponse<ArticleDetail>, String> {
    get_json(&format!("{API_BASE}/slug/{}", slug)).await
}

pub async fn get_article_by_title(title: &str) -> Result<AppResponse<Vec<ArticleMeta>>, String> {
    get_json(&format!("{API_BASE}/title/{}", title)).await
}

pub async fn get_article_page(page: u64, per: u64) -> Result<AppResponse<PageResult<ArticleMeta>>, String> {
    get_json(&format!("{API_BASE}/page?page={}&per={}", page, per)).await
}
