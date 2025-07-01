use canvord_service::article_query::ArticleQuery;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct FindArticleByIdHandler<'a> {
    db: &'a DbConn,
}

impl<'a> FindArticleByIdHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: i64) -> Result<ArticleDetail, AppError> {
        let model = ArticleQuery::find_article_by_id(self.db, id)
            .await?
            .ok_or_else(|| AppError::NotFound("article not found".into()))?;

        Ok(model.into())
    }
}