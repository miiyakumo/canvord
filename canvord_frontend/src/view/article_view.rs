use crate::api::{get_article_by_title, get_article_page};
use crate::component::{article_card::ArticleCard, pagination::Pagination};
use crate::model::Status;
use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;
use sycamore::web::{create_client_resource, Suspense};

#[component]
pub fn ArticleView() -> View {
    // 定义当前页数、每页大小、总页数，和文章状态
    let current_page = create_signal(1);
    let page_size = create_signal(10);
    let total_pages = create_signal(1); // 可以通过接口获取总页数
    let article_status = create_signal(None);
    let search_query = create_signal(String::new());

    // 获取文章数据
    let resource = create_client_resource({
        // 使用 move 确保将 current_page 捕获到异步闭包中
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let article_status = article_status.clone();
        move || {
            let page = current_page.get();
            let size = page_size.get();
            let status = article_status.get();
            async move { get_article_page(page, size, status).await.ok() }
        }
    });

    // 文章状态变化回调
    let on_status_change = move |status: Option<Status>| {
        article_status.set(status);
    };

    // 页码变化回调
    let on_page_change = move |page: i64| {
        current_page.set(page as u64);
    };

    // 页大小变化回调
    let on_size_change = move |size: i64| {
        page_size.set(size as u64);
    };

    // 搜索变化回调
    let on_search_change = Rc::new(move |query: String| {
        search_query.set(query);
    });

    // 发起搜索请求
    let search_articles = move || {
        let query = search_query.get_clone().trim().to_string();
        if !query.is_empty() {
            // 调用后端接口获取符合条件的文章
            get_article_by_title(&query);
        }
    };

    let on_select_change = move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Some(select) = target.dyn_into::<web_sys::HtmlSelectElement>().ok() {
                let selected_value = select.value().parse::<String>().unwrap_or("".to_string()); // 默认页大小 10
                let status = match selected_value.as_str() {
                    "Published" => Some(Status::Published),
                    "Unpublished" => Some(Status::Unpublished),
                    "Hidden" => Some(Status::Hidden),
                    _ => None,
                };
                on_status_change(status);
            }
        }
    };

    view! {
        div(class="flex items-center space-x-4") {
            // 搜索框
            div {
                input(
                    r#type="text",
                    on:keypress=move |event: web_sys::KeyboardEvent| {
                        if event.key() == "Enter" {
                            search_articles();
                        }
                    },
                    bind:value=search_query,
                    class="px-2 py-1 border rounded"
                )

                // 搜索按钮
                button(
                    on:click=move |_| search_articles(),
                    class="px-4 py-2 bg-blue-500 text-white rounded"
                ) { "搜索" }
            }

            // 文章状态选择框
            div {
                select(
                    on:change= Box::new(on_select_change),
                    class="px-2 py-1 border rounded"
                ) {
                    option(value="") { "全部" }
                    option(value="Published") { "已发布" }
                    option(value="Unpublished") { "未发布" }
                    option(value="Hidden") { "已隐藏" }
                }
            }
        }
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
                                                        println!("Article with id {} clicked", id);
                                                    })
                                                )
                                            }
                                        }
                                    )
                                }

                                Pagination(
                                    total_pages = total_pages.get() as i64,
                                    current_page = current_page.get() as i64,
                                    page_size = page_size.get() as i64,
                                    on_page_change = Rc::new(on_page_change),
                                    on_size_change = Rc::new(on_size_change),
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
