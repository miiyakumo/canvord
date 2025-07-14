use sea_orm::DatabaseConnection;
use std::sync::Arc;
use redis::Client;
use handler::{
    create_article_handler::CreateArticleHandler,
    delete_article_handler::DeleteArticleHandler,
    find_article_by_id_handler::FindArticleByIdHandler,
    find_article_by_slug_handler::FindArticleBySlugHandler,
    hide_article_handler::HideArticleHandler,
    list_article_by_title_handler::ListArticleByTitleHandler,
    list_articles_in_page_by_status_handler::ListArticlesInPageByStatusHandler,
    publish_article_handler::PublishArticleHandler,
    publish_draft_handler::PublishDraftHandler,
    save_article_handler::SaveArticleHandler,
    update_article_handler::UpdateArticleHandler,
    find_publish_article_by_slug_handler::FindPublishArticleBySlugHandler,
    list_publish_article_by_title_handler::ListPublishArticleByTitleHandler,
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis_client: Client,

    pub create_article: Arc<CreateArticleHandler<'static>>,
    pub update_article: Arc<UpdateArticleHandler<'static>>,
    pub delete_article: Arc<DeleteArticleHandler<'static>>,
    pub hide_article: Arc<HideArticleHandler<'static>>,
    pub publish_article: Arc<PublishArticleHandler<'static>>,
    pub publish_draft: Arc<PublishDraftHandler<'static>>,
    pub save_article: Arc<SaveArticleHandler<'static>>,
    pub find_by_id: Arc<FindArticleByIdHandler<'static>>,
    pub find_by_slug: Arc<FindArticleBySlugHandler<'static>>,
    pub find_publish_by_slug: Arc<FindPublishArticleBySlugHandler<'static>>,
    pub list_by_title: Arc<ListArticleByTitleHandler<'static>>,
    pub list_by_status_page: Arc<ListArticlesInPageByStatusHandler<'static>>,
    pub list_publish_by_title: Arc<ListPublishArticleByTitleHandler<'static>>,
}

impl AppState {
    pub fn new(db: Arc<DatabaseConnection>, redis_client: Client) -> Self {
        // NOTE: 用 `'static` 其实是因为 actix-web 的要求：必须线程安全 + 生命周期长。
        let db_ref: &'static DatabaseConnection = unsafe { std::mem::transmute::<&DatabaseConnection, &'static DatabaseConnection>(&*db) };

        Self {
            db,
            redis_client,
            create_article: Arc::new(CreateArticleHandler::new(db_ref)),
            update_article: Arc::new(UpdateArticleHandler::new(db_ref)),
            delete_article: Arc::new(DeleteArticleHandler::new(db_ref)),
            hide_article: Arc::new(HideArticleHandler::new(db_ref)),
            publish_article: Arc::new(PublishArticleHandler::new(db_ref)),
            publish_draft: Arc::new(PublishDraftHandler::new(db_ref)),
            save_article: Arc::new(SaveArticleHandler::new(db_ref)),
            find_by_id: Arc::new(FindArticleByIdHandler::new(db_ref)),
            find_by_slug: Arc::new(FindArticleBySlugHandler::new(db_ref)),
            find_publish_by_slug: Arc::new(FindPublishArticleBySlugHandler::new(db_ref)),
            list_by_title: Arc::new(ListArticleByTitleHandler::new(db_ref)),
            list_by_status_page: Arc::new(ListArticlesInPageByStatusHandler::new(db_ref)),
            list_publish_by_title: Arc::new(ListPublishArticleByTitleHandler::new(db_ref)),
        }
    }
}
