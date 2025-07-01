use chrono::NaiveDateTime;
use entity::article;

pub mod create_article_command;
pub mod update_article_command;
pub mod save_article_command;
pub mod delete_article_command;
pub mod hide_article_command;
pub mod publish_article_command;
pub mod publish_draft_command;

pub trait IntoActiveModel {
    fn into_active_model(self, now: NaiveDateTime) -> article::ActiveModel;
}