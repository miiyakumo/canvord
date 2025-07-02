use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;

/// 分页组件
#[component(inline_props)]
pub fn Pagination(
    total_pages: i64, // 总页数
    current_page: i64, // 当前页数
    page_size: i64, // 当前页大小
    on_page_change: Rc<dyn Fn(i64)>, // 页码变化回调，使用 Rc 来避免所有权转移
    on_size_change: Rc<dyn Fn(i64)>, // 页码变化回调，使用 Rc 来避免所有权转移
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

    // 页大小变化回调
    let change_page_size = {
        let on_size_change = on_size_change.clone();
        move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(select) = target.dyn_into::<web_sys::HtmlSelectElement>().ok() {
                    let selected_value = select.value().parse::<i64>().unwrap_or(10); // 默认页大小 10
                    on_size_change(selected_value); // 更新页大小
                }
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
            
            span { "每页：" }
            
            select(on:change=change_page_size) {
                option(value="8", selected=page_size == 8) { "8" }
                option(value="12", selected=page_size == 12) { "12" }
                option(value="16", selected=page_size == 16) { "16" }
                option(value="24", selected=page_size == 24) { "24" }
                option(value="48", selected=page_size == 48) { "48" }
            }
        }
    }
}
