use actix_web::web;
use di::{Injectable, ServiceCollection};
use sea_orm_migration::{migrator, MigratorTrait};
use crate::services::{sea_orm, DatabaseConnectionProvider, DatabaseConnectionProviderImpl};
use crate::services::sea_orm::{entity, DatabaseConnection};
use crate::services::sea_orm::sqlx::encode::IsNull::No;

pub fn add_custom_logger(
    mut builder: ServiceCollection,
) -> ServiceCollection {
    builder
}
pub fn add_custom_application_services(
    mut builder: ServiceCollection,
) -> ServiceCollection {
    let provider = builder
        .add(DatabaseConnectionProviderImpl::singleton())
        .build_provider()
        .unwrap();
    builder
}

pub fn add_custom_mediator(
    mut builder: ServiceCollection,
) -> ServiceCollection {
    builder
}

pub async fn apply_database_migration(provider: &DatabaseConnectionProviderImpl) {
    let db = provider.get_connection();
    migration::Migrator::up(&db, None).await.expect("");
}