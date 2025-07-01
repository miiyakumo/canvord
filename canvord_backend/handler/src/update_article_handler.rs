use canvord_service::article_mutation::ArticleMutation;
use command::update_article_command::UpdateArticleCommand;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct UpdateArticleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> UpdateArticleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }
    
    pub async fn execute(&self, cmd: UpdateArticleCommand) -> Result<ArticleDetail, AppError> {
        ArticleMutation::update(self.db, cmd)
            .await.map(|m| m.into())
            .map_err(|e| AppError::DbError(e.to_string()))
    }
}