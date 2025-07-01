use canvord_service::article_query::ArticleQuery;
use dto::app_error::AppError;
use dto::article::ArticleMeta;
use entity::article::Status;
use sea_orm::DbConn;

pub struct ListArticlesInPageByStatusHandler<'a> {
    db: &'a DbConn,
}

impl<'a> ListArticlesInPageByStatusHandler<'a> {
    pub fn new(db: &'a DbConn) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        page: u64,
        posts_per_page: u64,
        status: Option<Status>,
    ) -> Result<(Vec<ArticleMeta>, u64), AppError> {
        let (articles, num) = ArticleQuery::list_articles_in_page_by_status(
            self.db,
            page,
            posts_per_page,
            status
        ).await?;
        
        let article_metas: Vec<ArticleMeta> = articles
            .into_iter()
            .map(ArticleMeta::from)
            .collect();

        Ok((article_metas, num))
    }
}