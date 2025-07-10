use canvord_service::article_query::ArticleQuery;
use dto::app_error::AppError;
use dto::article::ArticleMeta;
use sea_orm::DbConn;

pub struct ListPublishArticleByTitleHandler<'a> {
    db: &'a DbConn,
}

impl<'a> ListPublishArticleByTitleHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }

    pub async fn execute(&self, title: String) -> Result<Vec<ArticleMeta>, AppError> {
        let title = format!("%{}%", title);
        let articles = ArticleQuery::list_publish_article_by_title(self.db, title).await?;

        Ok(articles.into_iter().map(ArticleMeta::from).collect())
    }
}