mod article_controller;
mod app_state;
mod util;
mod admin_controller;
mod visitor_controller;
mod api_info;

use crate::app_state::AppState;
use crate::article_controller::article_route;
use actix_files::Files as Fs;
use actix_web::middleware::Logger;
use actix_web::{web, HttpResponse, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::web::ServiceConfig;
use apistos::SwaggerUIConfig;
use migration::sea_orm::Database;
use migration::{Migrator, MigratorTrait};
use std::env;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::http::header;
use crate::admin_controller::admin_route;
use crate::api_info::api_info;
use crate::visitor_controller::visitor_route;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // let enable_swagger = env::var("ENABLE_SWAGGER")
    //     .unwrap_or_else(|_| "false".into())
    //     .to_lowercase() == "true";
    
    // establish connection to database and apply migrations
    let conn = Database::connect(&db_url).await.unwrap();
    let redis_client = redis::Client::open(redis_url.clone()).unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let app_state = AppState::new(Arc::from(conn), redis_client.clone());

    let server = HttpServer::new(move || {
        actix_web::App::new()
            .document(api_info())
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin() // 或 .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .supports_credentials() // 如果你用 cookie，必须加这个
                    .max_age(3600),
            )
            .configure(|cfg| {
                init_route(cfg, redis_client.clone());
            })
            .default_service(web::route().to(|| async {
                HttpResponse::Ok().body("404 Not Found")
            }))
            // .service(Fs::new("/static", "./api/static"))
            .build_with(
                    "/openapi.json",
                    BuildConfig::default()
                        .with(SwaggerUIConfig::new(&"/swagger")))
    });

    println!("Starting server at {server_url}");
    server
        .bind(server_url)?
        .run()
        .await?;

    Ok(())
}

fn init_route(cfg: &mut ServiceConfig, redis_client: redis::Client) {
    admin_route(cfg);
    article_route(cfg);
    visitor_route(cfg, redis_client);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}