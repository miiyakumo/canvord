use crate::api::{get_article_by_title, get_article_page};
use crate::component::{article_card::ArticleCard, pagination::Pagination};
use crate::model::{ArticleMeta, Status};
use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::futures::spawn_local;
use sycamore::web::wasm_bindgen::JsCast;
use sycamore::web::{create_client_resource, Suspense};
use sycamore_router::navigate;

#[derive(Debug, Clone, PartialEq)]
enum DisplayMode {
    Paginated,
    Search(Vec<ArticleMeta>),
}

#[component]
pub fn ArticleView() -> View {
    let current_page = create_signal(1);
    let page_size = create_signal(18);
    let total_pages = create_signal(1);
    let article_status = create_signal(None);
    let search_query = create_signal(String::new());

    let display_mode = create_signal(DisplayMode::Paginated);

    let resource = create_client_resource({
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

    let on_status_change = move |status: Option<Status>| {
        display_mode.set(DisplayMode::Paginated);
        article_status.set(status);
    };

    let on_page_change = move |page: i64| {
        current_page.set(page as u64);
    };

    let on_size_change = move |size: i64| {
        page_size.set(size as u64);
    };

    let search_articles = Rc::new(move || {
        let query = search_query.get_clone().trim().to_string();

        if query.is_empty() {
            display_mode.set(DisplayMode::Paginated);
            return;
        }

        let display_mode = display_mode.clone();
        spawn_local(async move {
            match get_article_by_title(&query).await {
                Ok(resp) => {
                    if resp.code == 0 {
                        let articles = resp.data.unwrap_or_else(Vec::new);
                        display_mode.set(DisplayMode::Search(articles));
                    } else {
                        gloo_console::error!("Search failed:", resp.message);
                        display_mode.set(DisplayMode::Search(vec![]));
                    }
                }
                Err(err) => {
                    gloo_console::error!("Error fetching search results:", err);
                    display_mode.set(DisplayMode::Search(vec![]));
                }
            }
        });
    });

    let on_select_change = move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Some(select) = target.dyn_into::<web_sys::HtmlSelectElement>().ok() {
                let selected_value = select.value();
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
        div(class="p-6 space-y-6") {
            div(class="bg-white shadow rounded-lg p-4 flex flex-wrap gap-4 items-center") {
                div(class="flex items-center gap-2") {
                    input(
                        r#type="text",
                        bind:value=search_query,
                        on:keypress={
                            let search_articles = search_articles.clone();
                            move |event: web_sys::KeyboardEvent| {
                                if event.key() == "Enter" {
                                    search_articles();
                                }
                            }
                        },
                        class="px-3 py-2 border border-gray-300 rounded w-64",
                        placeholder="搜索文章标题"
                    )
                    button(
                        on:click={
                            let search_articles = search_articles.clone();
                            move |_| search_articles()
                        },
                        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-semibold"
                    ) {
                        "搜索"
                    }
                }
    
                // 状态筛选
                div {
                    select(
                        on:change=Box::new(on_select_change),
                        class="px-3 py-2 border border-gray-300 rounded text-gray-700"
                    ) {
                        option(value="") { "全部状态" }
                        option(value="Published") { "已发布" }
                        option(value="Unpublished") { "未发布" }
                        option(value="Hidden") { "已隐藏" }
                    }
                }
            }
    
            (match display_mode.get_clone() {
                DisplayMode::Paginated => {
                    let res = resource.get_clone();
                    view! {
                        Suspense(fallback=|| view! { p(class="text-center text-gray-500") { "加载中..." } }) {
                            ({
                                if let Some(Some(resp)) = res.clone() {
                                    if resp.code == 0 {
                                        if let Some(page) = &resp.data {
                                            total_pages.set(page.total);
                                            view! {
                                                div(class="space-y-6") {
                                                    div(class="grid gap-6 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3") {
                                                        Indexed(
                                                            list=page.data.clone(),
                                                            view=|article| view! {
                                                                ArticleCard(
                                                                    article=article,
                                                                    on_click=Box::new(|id| navigate(&format!("article/edit/{}", id)))
                                                                )
                                                            }
                                                        )
                                                    }
                                                    div(class="flex justify-center mt-4") {
                                                        Pagination(
                                                            total_pages=total_pages.get() as i64,
                                                            current_page=current_page.get() as i64,
                                                            page_size=page_size.get() as i64,
                                                            on_page_change=Rc::new(on_page_change.clone()),
                                                            on_size_change=Rc::new(on_size_change.clone()),
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            view! { div(class="text-center text-gray-400 text-lg py-6") { "暂无文章数据" } }
                                        }
                                    } else {
                                        view! { div(class="bg-red-100 text-red-700 px-4 py-3 rounded text-center") { (resp.message.clone()) } }
                                    }
                                } else {
                                    view! { div(class="text-center text-gray-400 text-lg py-6") { "加载中或请求失败" } }
                                }
                            })
                        }
                    }
                },
                DisplayMode::Search(articles) => {
                    let list = create_memo(move || articles.clone());
                
                    view! {
                        div(class="space-y-6") {
                            h2(class="text-xl font-bold text-gray-700") { "搜索结果" }
                
                            (if list.get_clone().is_empty() {
                                view! {
                                    div(class="text-center text-gray-400 text-lg py-6") {
                                        "未找到相关文章"
                                    }
                                }
                            } else {
                                view! {
                                    div(class="grid gap-6 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3") {
                                        Indexed(
                                            list = list,
                                            view = |article| {
                                                view! {
                                                    ArticleCard(
                                                        article = article,
                                                        on_click = Box::new(|id| {
                                                            navigate(&*format!("article/edit/{}", id))
                                                        })
                                                    )
                                                }
                                            }
                                        )
                                    }
                                }
                            })
                        }
                    }
                }
            })
        }
    }
}