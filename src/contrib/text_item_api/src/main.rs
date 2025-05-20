use futures::executor::block_on;
use services::*;
use services::sea_orm::Iden;

async fn run() {
    let a = TextItemStorageImpl {}/* value */;
    a.add("111".to_string()).await;
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    block_on(run())
}