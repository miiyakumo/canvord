use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct DeleteArticleCommand {
    pub id: i64,
}