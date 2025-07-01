use canvord_service::article_mutation::ArticleMutation;
use command::publish_draft_command::PublishDraftCommand;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct PublishDraftHandler<'a> {
    db: &'a DbConn,
}

impl<'a> PublishDraftHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: PublishDraftCommand) -> Result<ArticleDetail, AppError> {
        ArticleMutation::publish_draft(self.db, cmd).await
            .map(|m| m.into())
            .map_err(|e| AppError::DbError(e.to_string()))
    }
}