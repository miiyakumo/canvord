use sycamore::prelude::*;
use crate::api::get_article_page;
use sycamore::web::{create_client_resource, Suspense};
use crate::component::article_card::ArticleCard;

#[component]
pub fn ArticleView() -> View {
    let resource = create_client_resource(|| async {
        get_article_page(1, 10, None).await.ok()
    });

    view! {
        Suspense(fallback=|| view! { p { "加载中..." } }) {
            (if let Some(Some(resp)) = resource.get_clone() {
                if resp.code == 0 {
                    if let Some(page) = resp.data {
                        view! {
                            div(class="grid grid-cols-1 md:grid-cols-2 gap-4") {
                                Indexed(
                                    list = page.data,
                                    view = |article| {
                                        view! {
                                            ArticleCard(
                                                article=article.clone(),
                                                on_click=Box::new(|id| {
                                                    console_log!("Article with id {} clicked", id);
                                                })
                                            )
                                        }
                                    }
                                )
                            }
                        }
                    } else {
                        view! { p { "无数据" } }
                    }
                } else {
                    view! { p { (format!("错误：{}", resp.message)) } }
                }
            } else {
                view! { p { "加载中或请求失败" } }
            })
        }
    }
}
