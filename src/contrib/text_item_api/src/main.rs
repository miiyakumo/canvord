mod main_functions;
mod commands;
mod services;
mod entity;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use compose_macro::compose;
use di::ServiceCollection;
use main_functions::*;
use crate::services::DatabaseConnectionProviderImpl;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    let provider_builder = compose!(
        add_custom_logger ->
        add_custom_application_services ->
        add_custom_mediator
    );
    
    let provider = provider_builder(ServiceCollection::new())
        .build_provider()
        .unwrap();
    
    apply_database_migration(&*provider.get::<DatabaseConnectionProviderImpl>().unwrap()).await;
    
     HttpServer::new(|| {
        App::new().service(
            web::scope("text_item_api")
        )
     }).bind("127.0.0.1:8080")?
         .run()
         .await
}