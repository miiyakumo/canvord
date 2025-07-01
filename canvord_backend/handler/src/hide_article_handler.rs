use canvord_service::article_mutation::ArticleMutation;
use command::hide_article_command::HideArticleCommand;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct HideArticleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> HideArticleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: HideArticleCommand) -> Result<ArticleDetail, AppError> {
        ArticleMutation::hide(self.db, cmd).await
            .map(|m| m.into())
            .map_err(|e| AppError::DbError(e.to_string()))
    }
}