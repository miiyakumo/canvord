use std::str::FromStr;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, Database, Set};
use sea_orm::prelude::Uuid;
use entity::text_item;
#[async_trait]
pub trait TextItemStorage {
    async fn add(&self, text: String) -> String;
}

pub struct TextItemStorageImpl {}

#[async_trait]
impl TextItemStorage for TextItemStorageImpl {
    async fn add(&self, text: String) -> String {
        let url = "mysql://root:Pass@word@canvord-sqldata:3306/text_item_api";
        let db = Database::connect(url).await.unwrap();
        
        let res = text_item::ActiveModel {
            content: Set(text),
            user_identity_uuid: Set(Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap()),
            ..Default::default()
        }.insert(&db).await.unwrap();
        res.id.to_string()
    }
}
