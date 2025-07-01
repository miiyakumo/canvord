use sycamore::prelude::*;
use crate::model::ArticleMeta;

#[component(inline_props)]
pub fn ArticleCard(article: ArticleMeta, on_click: Box<dyn Fn(i64) + 'static>) -> View {
    let id = article.id;
    let onclick = move |_| on_click(id);

    view! {
        div(
            class="bg-white shadow-md rounded-lg p-4 hover:shadow-xl transition cursor-pointer",
            on:click=onclick
        ) {
            h2(class="text-xl font-semibold text-indigo-600") { (article.title.clone()) }
            p(class="text-gray-600 mt-1 text-sm") { (article.description.clone()) }
            div(class="flex justify-between text-sm text-gray-500 mt-2") {
                span { (format!("分类: {}", article.category)) }
                span { (format!("更新: {}", article.last_update.format("%Y-%m-%d %H:%M"))) }
            }
        }
    }
}
