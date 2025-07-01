mod article_controller;
mod app_state;
mod util;

use crate::app_state::AppState;
use crate::article_controller::article_route;
use actix_files::Files as Fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::web::ServiceConfig;
use apistos::{api_operation, SwaggerUIConfig};
use migration::sea_orm::Database;
use migration::{Migrator, MigratorTrait};
use std::env;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::http::header;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let app_state = AppState::new(Arc::from(conn));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .document(api_info())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin() // 或 .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .supports_credentials() // 如果你用 cookie，必须加这个
                    .max_age(3600),
            )
            .configure(init)
            .default_service(web::route().to(not_found))
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(SwaggerUIConfig::new(&"/swagger")))
            .service(Fs::new("/static", "./api/static"))
    });

    println!("Starting server at {server_url}");
    server
        .bind(server_url)?
        .run()
        .await?;

    Ok(())
}

fn init(cfg: &mut ServiceConfig) {
    article_route(cfg);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

fn api_info() -> Spec{
    Spec {
        info: Info {
            title: "Rust Blog Web API".to_string(),
            description: Some(
                [
                    "这是一个基于 Rust 的博客系统后端 API。",
                    "",
                    "功能模块包括：",
                    "- 博客文章的创建、更新、删除与展示",
                    "- 图片上传和管理",
                    "- Markdown 渲染",
                    "- 用户认证与权限校验（JWT）",
                    "- 日志记录与访问管理",
                    "",
                    "本接口文档基于 Apistos 生成。",
                ].join("\n"),
            ),
            version: "v1.0.0".to_string(),
            ..Default::default()
        },
        servers: vec![Server {
            url: "/".to_string(),
            description: Some("Blog Web API Root".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[api_operation(summary = "not found")]
async fn not_found() -> impl Responder {
    HttpResponse::Ok().body("404 Not Found")
}
