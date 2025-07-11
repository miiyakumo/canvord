use std::borrow::Cow;
use pulldown_cmark::{Options, Parser};
use pulldown_cmark::html::push_html;
use sycamore::{component, view};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use crate::api::get_article_by_slug;
use crate::model::ArticleDetail;

#[derive(Clone)]
enum LoadStatus {
    Loading,
    Success,
    NotFound(String),
}

#[component]
pub fn ArticleView(slug: String) -> View {
    let article = create_signal(None::<ArticleDetail>);
    let html_content = create_signal(String::new());
    let load_status = create_signal(LoadStatus::Loading);

    spawn_local_scoped({
        let article = article.clone();
        let html_content = html_content.clone();
        let load_status = load_status.clone();

        async move {
            match get_article_by_slug(&slug).await {
                Ok(resp) => {
                    if let Some(detail) = resp.data {
                        let parser = Parser::new_ext(&detail.content_md, Options::all());
                        let mut html = String::new();
                        push_html(&mut html, parser);

                        html_content.set(html);
                        article.set(Some(detail));
                        load_status.set(LoadStatus::Success);
                    } else {
                        load_status.set(LoadStatus::NotFound("找不到该文章。".to_string()));
                    }
                }
                Err(err) => {
                    load_status.set(LoadStatus::NotFound(format!("加载文章失败：{}", err)));
                }
            }
        }
    });

    view! {
        div(class="min-h-screen bg-[url('/bg.webp')] bg-cover bg-fixed bg-center bg-no-repeat text-white flex justify-center px-4 py-8") {
            (match load_status.get_clone() {
                LoadStatus::Loading => view! {
                    div(class="text-lg text-neutral-400") { "加载中..." }
                },
                LoadStatus::Success => {
                    if let Some(article) = article.get_clone() {
                        let html: Cow<'static, str> = html_content.get_clone().clone().into();
                        view! {
                            div(class="bg-white text-neutral-900 max-w-3xl w-full p-8 rounded-xl shadow-lg bg-opacity-90") {
                                h1(class="text-3xl font-bold mb-4") { (article.meta.title.clone()) }
                                p(class="text-xs text-neutral-400 mb-4 italic") {
                                    "最后更新："
                                    (article.meta.last_update.format("%Y-%m-%d %H:%M:%S").to_string())
                                }
                                div(class="prose max-w-none", dangerously_set_inner_html=html)
                            }
                        }
                    } else {
                        view! { div() }
                    }
                },
                LoadStatus::NotFound(err) => view! {
                    div(class="text-lg text-red-400") {
                        (err)
                    }
                },
            })
        }
    }
}
