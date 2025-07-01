use crate::app_state::AppState;
use crate::util::{handle_api_result, validate};
use actix_web::{web, Responder};
use apistos::web as aweb;
use apistos::web::ServiceConfig;
use apistos::{api_operation, ApiComponent};
use command::create_article_command::CreateArticleCommand;
use command::delete_article_command::DeleteArticleCommand;
use command::hide_article_command::HideArticleCommand;
use command::publish_article_command::PublishArticleCommand;
use command::publish_draft_command::PublishDraftCommand;
use command::save_article_command::SaveArticleCommand;
use command::update_article_command::UpdateArticleCommand;
use entity::article::Status;
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;
use dto::app_response::AppResponse;

pub fn article_route(cfg: &mut ServiceConfig) {
    cfg.service(
        aweb::scope("/articles")
            .route("/create", aweb::post().to(create_article))
            .route("/update", aweb::put().to(update_article))
            .route("/delete", aweb::delete().to(delete_article))
            .route("/hide", aweb::put().to(hide_article))
            .route("/publish", aweb::put().to(publish_article))
            .route("/publish-draft", aweb::put().to(publish_draft))
            .route("/save-draft", aweb::put().to(save_draft))
            .route("/id/{id}", aweb::get().to(find_article_by_id))
            .route("/slug/{slug}", aweb::get().to(find_article_by_slug))
            .route("/title/{title}", aweb::get().to(list_article_by_title))
            .route("/page", aweb::get().to(list_articles))
    );
}

#[api_operation(summary = "创建文章")]
pub async fn create_article(
    data: web::Data<AppState>,
    payload: web::Json<CreateArticleCommand>,
) -> impl Responder {
    handle_api_result(data.create_article.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "修改文章")]
pub async fn update_article(
    data: web::Data<AppState>,
    payload: web::Json<UpdateArticleCommand>,
) -> impl Responder {
    handle_api_result(data.update_article.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "删除文章")]
pub async fn delete_article(
    data: web::Data<AppState>,
    payload: web::Json<DeleteArticleCommand>,
) -> impl Responder {
    handle_api_result(data.delete_article.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "隐藏文章")]
pub async fn hide_article(
    data: web::Data<AppState>,
    payload: web::Json<HideArticleCommand>,
) -> impl Responder {
    handle_api_result(data.hide_article.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "发布文章")]
pub async fn publish_article(
    data: web::Data<AppState>,
    payload: web::Json<PublishArticleCommand>,
) -> impl Responder {
    handle_api_result(data.publish_article.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "发布草稿")]
pub async fn publish_draft(
    data: web::Data<AppState>,
    payload: web::Json<PublishDraftCommand>,
) -> impl Responder {
    handle_api_result(data.publish_draft.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "保存草稿")]
pub async fn save_draft(
    data: web::Data<AppState>,
    payload: web::Json<SaveArticleCommand>,
) -> impl Responder {
    handle_api_result(data.save_article.execute(payload.into_inner()).await).await
}

#[api_operation(summary = "根据ID查询文章")]
pub async fn find_article_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> impl Responder {
    handle_api_result(data.find_by_id.execute(*id).await).await
}

#[api_operation(summary = "根据Slug查询文章")]
pub async fn find_article_by_slug(
    data: web::Data<AppState>,
    slug: web::Path<String>,
) -> impl Responder {
    handle_api_result(data.find_by_slug.execute(slug.clone()).await).await
}

#[api_operation(summary = "根据Title查询文章")]
pub async fn list_article_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> impl Responder {
    handle_api_result(data.list_by_title.execute(title.clone()).await).await
}

#[api_operation(summary = "分页查询文章（可选状态筛选）")]
pub async fn list_articles(
    data: web::Data<AppState>,
    query: web::Query<ArticlePageParams>,
) -> impl Responder {
    let params = query.into_inner();
    if let Err(e) = validate(&params) {
        return AppResponse::from_error(&e);
    }
    
    handle_api_result(data
        .list_by_status_page
        .execute(params.page, params.per, params.status)
        .await).await
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct ArticlePageParams {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 1, max = 100))]
    pub per: u64,
    pub status: Option<Status>,
}