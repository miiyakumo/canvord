use entity::article::{Column, Status};
use entity::{article, article::Entity as Article};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait, QueryOrder};

pub struct ArticleQuery;

impl ArticleQuery {
    pub async fn find_article_by_id(db: &DbConn, id: i64) -> Result<Option<article::Model>, DbErr> {
        Article::find_by_id(id).one(db).await
    }

    pub async fn find_article_by_slug(db: &DbConn, slug: String) -> Result<Option<article::Model>, DbErr> {
        Article::find_by_slug(&slug).one(db).await
    }
    
    pub async fn list_article_by_title(db: &DbConn, title: String) -> Result<Vec<article::Model>, DbErr> {
        Article::list_by_title(&title).order_by_asc(Column::CreatedAt).all(db).await
    }
    
    /// If ok, returns (article models, num pages).
    pub async fn list_articles_in_page_by_status(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
        status: Option<Status>,
    ) -> Result<(Vec<article::Model>, u64), DbErr> {
        let mut query = Article::find();

        if let Some(s) = status {
            query = query.filter(Column::Status.eq(s));
        }

        let paginator = query
            .order_by_asc(Column::CreatedAt)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}