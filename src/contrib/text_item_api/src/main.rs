mod main_functions;

use actix_web::{web, App, HttpServer};
use futures::executor::block_on;
use services::*;
use main_functions::*;

async fn run() {
    let a = TextItemStorageImpl {};
    a.add("111".to_string()).await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    HttpServer::new(|| {
        App::new().service(
            // 所有资源与路由加上前缀...
            web::scope("/app")
                .configure(apply_database_migration)
        )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}