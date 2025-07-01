use std::rc::Rc;
use sycamore::prelude::*;

/// 分页组件
#[component(inline_props)]
pub fn Pagination(
    total_pages: i64, // 总页数
    current_page: i64, // 当前页数
    on_page_change: Rc<dyn Fn(i64)>, // 页码变化回调，使用 Rc 来避免所有权转移
) -> View {
    // 上一页按钮回调
    let prev_page = {
        let on_page_change = on_page_change.clone();
        move |_| {
            if current_page > 1 {
                on_page_change(current_page - 1); // 调用回调函数
            }
        }
    };

    // 下一页按钮回调
    let next_page = {
        let on_page_change = on_page_change.clone();
        move |_| {
            if current_page < total_pages {
                on_page_change(current_page + 1); // 调用回调函数
            }
        }
    };

    view! {
        div(class="flex justify-center items-center space-x-2") {
            button(
                class="px-4 py-2 bg-gray-300 rounded-lg",
                on:click=prev_page,
                disabled=current_page == 1
            ) { "上一页" }

            span(class="px-4 py-2") { (format!("第 {} 页", current_page)) }

            button(
                class="px-4 py-2 bg-gray-300 rounded-lg",
                on:click=next_page,
                disabled=current_page == total_pages
            ) { "下一页" }
        }
    }
}
