mod commands;
mod entity;
mod main_functions;
mod services;

use actix_web::{get, route, web, App, HttpServer};
use dapr::dapr::proto::runtime::v1::dapr_client::DaprClient;
use main_functions::*;

#[get("/hello")]
async fn hello() -> String {
    "Hello, world!".to_string()
}
use std::time::Duration;

async fn connect_dapr_with_retry() -> DaprClient<tonic::transport::Channel> {
    let mut retries = 10;
    loop {
        match DaprClient::connect("http://127.0.0.1:50001").await {
            Ok(client) => return client,
            Err(e) => {
                if retries == 0 {
                    panic!("Failed to connect to Dapr sidecar: {:?}", e);
                }
                eprintln!("Waiting for Dapr sidecar... ({:?})", e);
                retries -= 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    println!("Starting Text Item API");

    let mut client = connect_dapr_with_retry().await;
    use dapr::dapr::proto::runtime::v1::GetSecretRequest;

    let request = GetSecretRequest {
        store_name: "canvord-secretstore".to_string(),
        key: "ConnectionStrings:TextItemDb".to_string(),
        metadata: Default::default(),
    };

    let response = client.get_secret(request).await.unwrap();
    let secret_map = response.into_inner().data;
    let url = secret_map.get("ConnectionStrings:TextItemDb");

    apply_database_migration(url.unwrap()).await;
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}