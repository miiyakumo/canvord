use command::create_article_command::CreateArticleCommand;
use command::delete_article_command::DeleteArticleCommand;
use command::hide_article_command::HideArticleCommand;
use command::publish_article_command::PublishArticleCommand;
use command::publish_draft_command::PublishDraftCommand;
use command::save_article_command::SaveArticleCommand;
use command::update_article_command::UpdateArticleCommand;
use command::IntoActiveModel;
use entity::article::Status;
use entity::{article, article::Entity as Article};
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, DeleteResult, Set};

pub struct ArticleMutation;

impl ArticleMutation {
    pub async fn create(db: &DbConn, cmd: CreateArticleCommand) -> Result<article::ActiveModel, DbErr> {
        let now = Utc::now().naive_utc();
        cmd.into_active_model(now).save(db).await
    }

    pub async fn save_draft(db: &DbConn, cmd: SaveArticleCommand) -> Result<article::ActiveModel, DbErr> {
        let now = Utc::now().naive_utc();
        cmd.into_active_model(now).save(db).await
    }

    pub async fn delete(db: &DbConn, cmd: DeleteArticleCommand) -> Result<DeleteResult, DbErr> {
        let article = Self::find_active_model(db, cmd.id).await?;
        article.delete(db).await
    }

    pub async fn update(db: &DbConn, cmd: UpdateArticleCommand) -> Result<article::Model, DbErr> {
        let now = Utc::now().naive_utc();
        cmd.into_active_model(now).update(db).await
    }

    pub async fn publish(db: &DbConn, cmd: PublishArticleCommand) -> Result<article::Model, DbErr> {
        let article = Self::find_active_model(db, cmd.id).await?;
        Self::apply_status(article, Status::Published).update(db).await
    }

    pub async fn publish_draft(db: &DbConn, cmd: PublishDraftCommand) -> Result<article::Model, DbErr> {
        // 先获取当前文章 ActiveModel
        let article = Self::find_active_model(db, cmd.id).await?;

        // 业务约束：仅允许草稿状态的文章发布草稿
        if article.status.as_ref().ne(&Status::Unpublished){
            return Err(DbErr::Custom("Only draft articles can be published".to_owned()));
        }

        let now = Utc::now().naive_utc();
        cmd.into_active_model(now).update(db).await
    }

    pub async fn hide(db: &DbConn, cmd: HideArticleCommand) -> Result<article::Model, DbErr> {
        let article = Self::find_active_model(db, cmd.id).await?;
        Self::apply_status(article, Status::Hidden).update(db).await
    }

    fn apply_status(mut model: article::ActiveModel, status: Status) -> article::ActiveModel {
        model.status = Set(status);
        model.last_update = Set(Utc::now().naive_utc());
        model
    }

    async fn find_active_model(db: &DbConn, id: i64) -> Result<article::ActiveModel, DbErr> {
        Article::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find article".to_owned()))
            .map(Into::into)
    }
}
