use actix_web::web;
use sea_orm::Database;
use sea_orm_migration::{migrator, MigratorTrait};
use sea_orm::{entity, DatabaseConnection};
use sea_orm::sqlx::encode::IsNull::No;

pub async fn apply_database_migration(url: &String) {
    let db = Database::connect(url).await.expect("Failed to connect to the database");
    migration::Migrator::up(&db, None).await.expect("");
}