use std::rc::Rc;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::console_error;
use sycamore::web::js_sys::Math;
use sycamore::web::rt::web_sys;
use sycamore_router::navigate;
use crate::api::{get_article_by_slug, get_article_by_title, get_article_page};
use crate::model::{ArticleDetail, ArticleMeta};


#[derive(Clone, PartialEq)]
enum DisplayMode {
    Paginated,
    Search(Vec<ArticleMeta>),
}

#[component]
pub fn HomeView() -> View {
    let current_page = create_signal(1);
    let page_size = create_signal(20);
    let total_pages = create_signal(1);
    let search_query = create_signal(String::new());
    let articles = create_signal(Vec::new());
    let display_mode = create_signal(DisplayMode::Paginated);

    let fetch_paginated = {
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let articles = articles.clone();
        let total_pages = total_pages.clone();
        let display_mode = display_mode.clone();
        move || {
            display_mode.set(DisplayMode::Paginated);
            let page = current_page.get();
            let size = page_size.get();
            spawn_local_scoped(async move {
                match get_article_page(page, size).await {
                    Ok(resp) => {
                        if let Some(data) = resp.data {
                            total_pages.set(data.total);
                            articles.set(data.data);
                        }
                    }
                    Err(err) => console_error!("获取文章失败: {}", err),
                }
            });
        }
    };

    fetch_paginated();

    let search_articles = Rc::new(move || {
        let query = search_query.get_clone().trim().to_string();
        if query.is_empty() {
            fetch_paginated();
            return;
        }

        let display_mode = display_mode.clone();
        spawn_local_scoped(async move {
            match get_article_by_title(&query).await {
                Ok(resp) => {
                    let result = resp.data.unwrap_or_default();
                    display_mode.set(DisplayMode::Search(result));
                }
                Err(err) => {
                    console_error!("搜索失败: {}", err);
                    display_mode.set(DisplayMode::Search(vec![]));
                }
            }
        });
    });

    let on_select = Rc::new(move |slug: String| {
        navigate(&format!("/article/{}", slug));
    });


    view! {
        div(class="grid h-screen w-screen bg-neutral-900 grid-cols-[7fr_25px_1fr] grid-rows-[3fr_15px_1fr]") {
            // 左上区块：文章列表
            div(class="bg-black p-4 h-full overflow-y-auto") {
                CardList(
                    articles=articles.clone(),
                    on_select=on_select
                )
            }

            // 右上区块：搜索与个人信息
            div(class="grid grid-rows-[1fr_2fr] gap-2 h-full w-full") {
                // 顶部：搜索与分页
                div(class="bg-orange-200 p-4 flex flex-col space-y-4") {
                    // 搜索栏
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
                
                    // 分页控制栏
                    div(class="flex flex-wrap items-center gap-3 text-sm text-black") {
                        button(
                            class="bg-red-600 text-white px-4 py-1 border-2 border-black hover:bg-red-700",
                            on:click=move |_| {
                                if current_page.get() > 1 {
                                    current_page.set(current_page.get() - 1);
                                    fetch_paginated();
                                }
                            }
                        ) { "上一页" }
                
                        button(
                            class="bg-blue-600 text-white px-4 py-1 border-2 border-black hover:bg-blue-700",
                            on:click=move |_| {
                                if current_page.get() < total_pages.get() as u64 {
                                    current_page.set(current_page.get() + 1);
                                    fetch_paginated();
                                }
                            }
                        ) { "下一页" }
                
                        span(class="text-sm") {
                            (format!("第 {} 页 / 共 {} 页", current_page.get(), total_pages.get()))
                        }
                
                        input(
                            r#type="number",
                            min="1",
                            max=total_pages.get().to_string(),
                            class="w-16 px-2 py-1 border border-black text-black",
                            on:change=move |e| {
                                // if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                //     if let Ok(page) = input.value().parse::<usize>() {
                                //         if page >= 1 && page <= total_pages.get() {
                                //             current_page.set(page);
                                //             fetch_paginated();
                                //         }
                                //     }
                                // }
                            }
                        )
                    }
                }

                // 底部：个人介绍
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


fn random_index(max: usize) -> usize {
    (Math::random() * max as f64).floor() as usize
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
                    // 随机取颜色
                    let idx = (Math::random() * color_pool.len() as f64).floor() as usize;
                    let color_class = color_pool[idx];
                    let slug = article.slug.clone();

                    // 这里先克隆slug，让闭包可以FnMut多次调用
                    let on_select = on_select.clone();
                    view! {
                        div(
                            class=format!("{} shadow-sm p-2 flex flex-col justify-between cursor-pointer", color_class),
                            on:click=move |_| {
                                let slug_clone = slug.clone();
                                on_select(slug_clone);
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
