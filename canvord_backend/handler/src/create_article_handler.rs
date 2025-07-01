use canvord_service::article_mutation::ArticleMutation;
use command::create_article_command::CreateArticleCommand;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct CreateArticleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> CreateArticleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: CreateArticleCommand) -> Result<ArticleDetail, AppError> {
        let active_model = ArticleMutation::create(self.db, cmd).await?;
        let detail: ArticleDetail = active_model.try_into()?;
        Ok(detail)
    }
}