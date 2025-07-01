use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct PublishArticleCommand {
    pub id: i64,
}