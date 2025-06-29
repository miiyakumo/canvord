use actix_files::Files as Fs;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use migration::{Migrator, MigratorTrait};
use migration::sea_orm::Database;
use utoipa::OpenApi;
use std::env;
use utoipa_swagger_ui::SwaggerUi;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

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
    // -> create post table if not exists
    println!("{}", db_url.as_str());
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/static", "./api/static"))
            .wrap(middleware::Logger::default()) // enable logger
            // .default_service(web::route().to(hello))
            .configure(init)
    });

    println!("Starting server at {server_url}");
    server
        .bind(server_url)?
        .run()
        .await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

#[derive(OpenApi)]
#[openapi(paths(hello), components(schemas(HelloResponse)))]
struct ApiDoc;
