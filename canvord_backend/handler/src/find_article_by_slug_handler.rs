use canvord_service::article_query::ArticleQuery;
use dto::app_error::AppError;
use dto::article::ArticleDetail;
use sea_orm::DbConn;

pub struct FindArticleBySlugHandler<'a> {
    db: &'a DbConn,
}

impl<'a> FindArticleBySlugHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }

    pub async fn execute(&self, slug: String) -> Result<ArticleDetail, AppError> {
        let model = ArticleQuery::find_article_by_slug(self.db, slug)
            .await?
            .ok_or_else(|| AppError::NotFound("article not found".into()))?;

        Ok(model.into())
    }
}