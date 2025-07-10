use std::rc::Rc;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::console_error;
use sycamore::web::js_sys::Math;
use sycamore_router::navigate;
use crate::api::{get_article_by_slug, get_article_page};
use crate::model::{ArticleDetail, ArticleMeta};

#[component]
pub fn HomeView() -> View {
    let articles = create_signal(vec![]);
    let selected_article = create_signal(None::<ArticleDetail>);

    spawn_local_scoped({
        let articles = articles.clone();
        async move {
            match get_article_page(1, 99).await {
                Ok(resp) => {
                    if let Some(data) = resp.data {
                        articles.set(data.data);
                    }
                }
                Err(err) => console_error!("获取文章列表失败: {}", err),
            }
        }
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

            // 右上区块：显示选中文章内容
            div(class="bg-slate-500 p-8 flex flex-col justify-start items-start overflow-y-auto") {
                (if let Some(article) = selected_article.get_clone() {
                    view! {
                        div {
                            h2(class="text-3xl font-bold text-white") { (article.meta.title.clone()) }
                            p(class="text-sm text-white mt-2") { (article.meta.description.clone()) }
                            div(class="mt-4 text-white whitespace-pre-wrap") { (article.content_md.clone()) }
                        }
                    }
                } else {
                    view! {
                        div(class="text-white") { "我是我" }
                    }
                })
            }

            // 其他区块保持不变
            div(class="bg-yellow-300 p-8 flex flex-col justify-center items-center") {
                h2(class="text-2xl font-bold text-neutral-800") { "还没想好要说什么" }
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
