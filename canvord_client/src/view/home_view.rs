use std::rc::Rc;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::{console_error, js_sys};
use sycamore::web::rt::web_sys;
use sycamore::web::wasm_bindgen::JsCast;
use sycamore_router::navigate;
use crate::api::{get_article_by_title, get_article_page};
use crate::model::{ArticleMeta};

#[derive(Clone, PartialEq, Debug)]
enum DisplayMode {
    Paginated,
    Search,
}

#[component]
pub fn HomeView() -> View {
    let current_page = create_signal(1);
    let page_size = create_signal(42);
    let total_pages = create_signal(1);
    let search_query = create_signal(String::new());
    let articles = create_signal(Vec::<ArticleMeta>::new());
    let display_mode = create_signal(DisplayMode::Paginated);

    // Effect for fetching paginated articles when page or mode changes.
    create_effect( move || {
        // Only fetch if in paginated mode.
        if display_mode.get_clone() != DisplayMode::Paginated {
            return;
        }

        // This effect depends on current_page, so it re-runs when the page changes.
        let page = current_page.get();
        let size = page_size.get();

        spawn_local_scoped( async move {
            match get_article_page(page, size).await {
                Ok(resp) => {
                    if let Some(data) = resp.data {
                        // Assuming data.total is the number of pages.
                        total_pages.set(data.total);
                        articles.set(data.data);
                    }
                }
                Err(err) => console_error!("获取文章失败: {}", err),
            }
        });
    });

    // Action for handling search.
    let search_articles = move || {
        let query = search_query.get_clone().trim().to_string();
        if query.is_empty() {
            display_mode.set(DisplayMode::Paginated);
            return;
        }

        display_mode.set(DisplayMode::Search);
        spawn_local_scoped(async move {
            match get_article_by_title(&query).await {
                Ok(resp) => {
                    articles.set(resp.data.unwrap_or_default());
                }
                Err(err) => {
                    console_error!("搜索失败: {}", err);
                    articles.set(vec![]);
                }
            }
        });
    };

    // Callback for when a card is selected.
    let on_select = Rc::new(move |slug: String| {
        navigate(&format!("/article/{}", slug));
    });

    view! {
        div(class="grid h-screen w-screen bg-neutral-900 grid-cols-[7fr_25px_1fr] grid-rows-[3fr_15px_1fr]") {
            // Left-Top Block: Article List
            div(class="bg-black p-4 h-full overflow-y-auto") {
                CardList(articles=articles, on_select=on_select)
            }

            // Right-Top Block: Search & Info
            div(class="grid grid-rows-[1fr_2fr] gap-2 h-full w-full") {
                // Top part: Search and Pagination
                div(class="bg-orange-200 p-4 flex flex-col space-y-4") {
                    div(class="flex space-x-2") {
                        input(
                            placeholder="搜索文章标题...",
                            bind:value=search_query,
                            class="w-full px-3 py-1 bg-white text-black border-2 border-black shadow-none focus:outline-none focus:ring-0 placeholder-gray-600",
                            on:keypress=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" {
                                    search_articles();
                                }
                            }
                        )
                        button(class="bg-gray-700 text-white px-4 py-1 border-2 border-black hover:bg-gray-800", on:click=move |_| search_articles()) { "搜索" }
                    }

                    (if display_mode.get_clone() == DisplayMode::Paginated {
                        view! {
                            div(class="flex flex-wrap items-center gap-3 text-sm text-black") {
                                // 上一页按钮
                                button(
                                    class="bg-red-600 text-white px-4 py-1 border-2 border-black hover:bg-red-700 disabled:opacity-50",
                                    disabled=current_page.get() <= 1,
                                    on:click=move |_| {
                                        current_page.set(current_page.get() - 1);
                                    }
                                ) { "上一页" }
                            
                                // 下一页按钮
                                button(
                                    class="bg-blue-600 text-white px-4 py-1 border-2 border-black hover:bg-blue-700 disabled:opacity-50",
                                    disabled=current_page.get() >= total_pages.get() as u64,
                                    on:click=move |_| {
                                        current_page.set(current_page.get() + 1);
                                    }
                                ) { "下一页" }
                            
                                // 跳页输入框
                                input(
                                    r#type="number",
                                    min="1",
                                    max=total_pages.get().to_string(),
                                    class="w-16 px-2 py-1 border border-black text-black",
                                    placeholder="页码",
                                    on:change=move |e: web_sys::Event| {
                                        if let Some(input) = e.target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                            if let Ok(page) = input.value().parse::<usize>() {
                                                if page >= 1 && page <= total_pages.get() {
                                                    current_page.set(page as u64);
                                                }
                                            }
                                        }
                                    }
                                )
                            
                                // 选择页面大小
                                select(
                                    class="px-2 py-1 border border-black text-black bg-white",
                                    on:change=move |e: web_sys::Event| {
                                        if let Some(select) = e.target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                                            if let Ok(size) = select.value().parse::<usize>() {
                                                page_size.set(size as u64);
                                                current_page.set(1); // 重置到第一页
                                            }
                                        }
                                    }
                                ) {
                                    option(value="10", selected=page_size.get() == 10) { "每页10" }
                                    option(value="20", selected=page_size.get() == 20) { "每页20" }
                                    option(value="42", selected=page_size.get() == 42) { "每页42" }
                                    option(value="100", selected=page_size.get() == 100) { "每页100" }
                                }
                            
                                // 当前页 / 总页数
                                span(class="text-sm") {
                                    (format!("第 {} 页 / 共 {} 页", current_page.get(), total_pages.get()))
                                }
                            }
                        }
                    } else {
                        view! { div(class="text-sm text-black") { "搜索结果" } }
                    })
                }

                // Bottom part: Personal Intro
                div(class="bg-blue-300 p-6 flex flex-col justify-center items-start text-neutral-900 space-y-2") {
                    h2(class="text-xl font-bold") { "关于本站" }
                    p(class="text-sm leading-relaxed") {
                        "这个网站是用 Sycamore + Rust 构建的玩具项目，整个项目Rust含量达到99%。"
                    }
                    p(class="text-sm text-neutral-700 italic") {
                        "“致敬传奇画家蒙德里安”"
                    }
                }
            }

            // Bottom-Left Block
            div(class="bg-yellow-300 p-10 shadow-inner flex flex-col items-center text-neutral-900 space-y-6") {
                h2(class="text-2xl font-extrabold text-center leading-relaxed") {
                    "你在这个网站遇到的所有丑陋和不便\n可以归于以下两点："
                }

                ol(class="list-decimal list-inside space-y-2 text-lg font-medium") {
                    li { "我不会前端。" }
                    li { "写好了也没人看。" }
                }

                p(class="text-sm text-neutral-700 italic pt-2") {
                    "（ “但我会尽量让它没那么糟 ” by GPT）"
                }
            }
            
            div(class="bg-white p-8 flex flex-col justify-center items-center") {
                img(src="logo.svg", alt="Logo", class="w-full h-full max-w-full max-h-full object-contain")
            }
            
            div(class="col-start-2 row-span-full bg-red-700")
            div(class="row-start-2 col-span-full bg-blue-600")
        }
    }
}

#[component(inline_props)]
pub fn CardList(
    articles: Signal<Vec<ArticleMeta>>,
    on_select: Rc<dyn Fn(String)>
) -> View {
    let color_pool = [
        "bg-yellow-400", "bg-blue-300", "bg-red-300",
        "bg-indigo-400", "bg-emerald-300", "bg-pink-400",
        "bg-orange-300", "bg-lime-400", "bg-cyan-300",
        "bg-purple-300", "bg-rose-400"
    ];

    view! {
        div(class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2") {
            Indexed(
                list=articles,
                view=move |article| {
                    let idx = (js_sys::Math::random() * color_pool.len() as f64).floor() as usize;
                    let color_class = color_pool[idx];
                    let slug = article.slug.clone();
                    // 这里先克隆slug，让闭包可以FnMut多次调用 
                    let on_select = on_select.clone();

                    view! {
                        div(
                            class=format!("{} shadow-sm p-2 flex flex-col justify-between cursor-pointer", color_class),
                            on:click=move |_| {
                                on_select(slug.clone());
                            }
                        ) {
                            h3(class="font-semibold text-base") { (article.title.clone()) }
                            p(class="text-sm text-neutral-800") { (article.description.clone()) }
                        }
                    }
                }
            )
        }
    }
}