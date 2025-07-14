use actix_web::{web, Responder};
use apistos::{web as aweb, ApiComponent};
use apistos::api_operation;
use apistos::web::ServiceConfig;
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;
use dto::app_response::AppResponse;
use dto::pagination::PageResult;
use entity::article::Status;
use middleware::cache::CacheMiddleware;
use crate::app_state::AppState;
use crate::util::{handle_api_result, validate};

pub fn visitor_route(cfg: &mut ServiceConfig, redis_client: redis::Client) {
    cfg.service(
        aweb::scope("/visitor")
            .wrap(
                CacheMiddleware::new(redis_client)
                    .with_ttl(666)
                    .with_key_gen(|req| {
                        let uri = req.uri();
                        let path = uri.path();
                        let mut key = path.to_string();
            
                        if let Some(query) = uri.query() {
                            // 只保留有效的 k=v 格式参数
                            let valid_params: Vec<_> = query
                                .split('&')
                                .filter(|s| s.contains('=') && s.split('=').count() == 2)
                                .collect();
                            if !valid_params.is_empty() {
                                key.push('?');
                                key.push_str(&valid_params.join("&"));
                            }
                        }
            
                        key
                    })
            )
            .route("/slug/{slug}", aweb::get().to(find_article_by_slug))
            .route("/title/{title}", aweb::get().to(list_article_by_title))
            .route("/page", aweb::get().to(list_articles))
    );
}

#[api_operation(summary = "根据Slug查询公开文章")]
pub async fn find_article_by_slug(
    data: web::Data<AppState>,
    slug: web::Path<String>,
) -> impl Responder {
    handle_api_result(data.find_publish_by_slug.execute(slug.clone()).await).await
}

#[api_operation(summary = "根据Title查询公开文章")]
pub async fn list_article_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> impl Responder {
    handle_api_result(data.list_publish_by_title.execute(title.clone()).await).await
}

#[api_operation(summary = "分页查询公开文章")]
pub async fn list_articles(
    data: web::Data<AppState>,
    query: web::Query<ArticlePageParams>,
) -> impl Responder {
    let params = query.into_inner();
    if let Err(e) = validate(&params) {
        return AppResponse::from_error(&e);
    }

    let handler = &data.list_by_status_page;
    match handler.execute(params.page, params.per, Option::from(Status::Published)).await {
        Ok((articles, total)) => {

            let page_result = PageResult {
                total: total as usize,
                current: params.page as usize,
                size: params.per as usize,
                data: articles,
            };
            AppResponse::ok(page_result)
        }
        Err(e) => AppResponse::from_error(&e),
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct ArticlePageParams {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 1, max = 100))]
    pub per: u64,
}