use std::env;
use actix_web::{web, HttpResponse, Responder};
use apistos::{web as aweb, ApiComponent};
use apistos::api_operation;
use apistos::web::ServiceConfig;
use schemars::JsonSchema;
use serde::Deserialize;
use dto::app_response::AppResponse;
use crate::util::create_jwt;

pub fn admin_route(cfg: &mut ServiceConfig) {
    cfg.service(
        aweb::scope("/admin")
            .route("/login", aweb::post().to(login))
    );
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[api_operation(summary = "管理员登录")]
pub async fn login(
    form: web::Json<LoginRequest>,
) -> impl Responder {
    dotenvy::dotenv().ok(); // 加载 .env 变量

    let expected_user = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".into());
    let expected_pass = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "123456".into());

    if form.username == expected_user && form.password == expected_pass {
        match create_jwt(&form.username, "admin") {
            Ok(token) => AppResponse::ok(token),
            Err(_) => AppResponse::message(500, "Token 创建失败"),
        }
    } else {
        AppResponse::message(401, "用户名或密码错误")
    }
}