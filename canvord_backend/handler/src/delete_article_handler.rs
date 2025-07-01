use canvord_service::article_mutation::ArticleMutation;
use command::delete_article_command::DeleteArticleCommand;
use dto::app_error::AppError;
use sea_orm::DbConn;

pub struct DeleteArticleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> DeleteArticleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: DeleteArticleCommand) -> Result<u64, AppError> {
        Ok(ArticleMutation::delete(self.db, cmd).await?.rows_affected)
    }
}