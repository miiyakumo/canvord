use canvord_service::article_mutation::ArticleMutation;
use command::publish_article_command::PublishArticleCommand;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct PublishArticleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> PublishArticleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: PublishArticleCommand) -> Result<ArticleDetail, AppError> {
        ArticleMutation::publish(self.db, cmd).await
            .map(|m| m.into())
            .map_err(|e| AppError::DbError(e.to_string()))
    }
}