use actix_web::get;

#[get("/bye")]
async fn bye() -> String {
    "Bye, world!".to_string()
}