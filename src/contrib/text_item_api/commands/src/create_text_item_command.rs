use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatTextItemCommand {
    pub content: String,
}