use sycamore::prelude::*;
use crate::api::get_article_page;
use sycamore::web::{create_client_resource, Suspense};
use crate::component::{article_card::ArticleCard, pagination::Pagination};
use std::rc::Rc;

#[component]
pub fn ArticleView() -> View {
    // 定义当前页数和总页数
    let current_page = create_signal(1);
    let total_pages = create_signal(1); // 可以通过接口获取总页数

    // 获取文章数据
    let resource = create_client_resource({
        // 使用 move 确保将 current_page 捕获到异步闭包中
        let current_page = current_page.clone();
        move || {
            // 在异步闭包外部获取 page，避免在闭包内引用 current_page
            let page = current_page.get();
            async move {
                get_article_page(page, 4, None).await.ok()
            }
        }
    });

    view! {
        Suspense(fallback=|| view! { p { "加载中..." } }) {
            (if let Some(Some(resp)) = resource.get_clone() {
                if resp.code == 0 {
                    if let Some(page) = resp.data {
                        // 更新总页数
                        total_pages.set(page.total);

                        view! {
                            div(class="space-y-4") {
                                div(class="grid grid-cols-1 md:grid-cols-2 gap-4") {
                                    Indexed(
                                        list = page.data,
                                        view = |article| {
                                            view! {
                                                ArticleCard(
                                                    article = article.clone(),
                                                    on_click = Box::new(|id| {
                                                        // 跳转到文章编辑页面或其他操作
                                                        println!("Article with id {} clicked", id);
                                                    })
                                                )
                                            }
                                        }
                                    )
                                }

                                // 分页组件
                                Pagination(
                                    total_pages = total_pages.get() as i64, // 总页数
                                    current_page = current_page.get() as i64, // 当前页数
                                    on_page_change = Rc::new(move |page| {
                                        current_page.set(page as u64); // 更新当前页数
                                    })
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
