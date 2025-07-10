// src/api/auth.rs
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::model::AppResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(data: &LoginRequest) -> Result<AppResponse<String>, String> {
    Request::post("http://localhost:8000/admin/login")
        .header("Content-Type", "application/json")
        .json(data)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub fn logout() {
    use gloo_storage::{Storage, LocalStorage};
    let _ = LocalStorage::delete("jwt_token");
}

pub fn store_token(token: &str) {
    use gloo_storage::{Storage, LocalStorage};
    let _ = LocalStorage::set("jwt_token", token);
}

pub fn load_token() -> Option<String> {
    use gloo_storage::{Storage, LocalStorage};
    LocalStorage::get("jwt_token").ok()
}
