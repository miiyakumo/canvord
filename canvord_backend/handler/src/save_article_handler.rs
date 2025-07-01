use canvord_service::article_mutation::ArticleMutation;
use command::save_article_command::SaveArticleCommand;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct SaveArticleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> SaveArticleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: SaveArticleCommand) -> Result<ArticleDetail, AppError> {
        let active_model = ArticleMutation::save_draft(self.db, cmd).await?;
        let detail: ArticleDetail = active_model.try_into()?;
        Ok(detail)
    }
}