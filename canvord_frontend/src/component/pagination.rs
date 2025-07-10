use std::rc::Rc;
use sycamore::prelude::*;
use sycamore::web::console_warn;
use sycamore::web::wasm_bindgen::JsCast;

/// 分页组件
#[component(inline_props)]
pub fn Pagination(
    total_pages: i64, // 总页数
    current_page: i64, // 当前页数
    page_size: i64, // 当前页大小
    on_page_change: Rc<dyn Fn(i64)>, // 页码变化回调
    on_size_change: Rc<dyn Fn(i64)>, // 页大小变化回调
) -> View {
    // NEW: 创建一个本地 signal 来管理跳转输入框的值
    let jump_to_page = create_signal(String::new());

    // 上一页按钮回调
    let prev_page = {
        let on_page_change = on_page_change.clone();
        move |_| {
            if current_page > 1 {
                on_page_change(current_page - 1);
            }
        }
    };

    // 下一页按钮回调
    let next_page = {
        let on_page_change = on_page_change.clone();
        move |_| {
            if current_page < total_pages {
                on_page_change(current_page + 1);
            }
        }
    };

    // 页大小变化回调
    let change_page_size = {
        let on_size_change = on_size_change.clone();
        move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(select) = target.dyn_into::<web_sys::HtmlSelectElement>().ok() {
                    let selected_value = select.value().parse::<i64>().unwrap_or(18);
                    on_size_change(selected_value);
                }
            }
        }
    };

    // NEW: 跳转按钮的回调逻辑
    let go_to_page = {
        let on_page_change = on_page_change.clone();
        move |_| {
            if let Ok(page_num) = jump_to_page.get_clone().parse::<i64>() {
                // 验证输入值是否在有效范围内
                if page_num > 0 && page_num <= total_pages {
                    on_page_change(page_num);
                } else {
                    // (可选) 如果输入无效，可以给一些提示或重置输入框
                    console_warn!("输入的页码无效: {}", page_num);
                }
            }
        }
    };
    
    view! {
        div(class="flex justify-center items-center space-x-2") {
            button(
                class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed",
                on:click=prev_page,
                disabled=current_page == 1
            ) { "上一页" }

            span(class="px-4 py-2 text-gray-700") { 
                (format!("第 {} / {} 页", current_page, total_pages)) 
            }

            button(
                class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed",
                on:click=next_page,
                disabled=current_page >= total_pages
            ) { "下一页" }
            
            // NEW: 跳转功能 UI
            div(class="flex items-center space-x-2 ml-4") {
                input(
                    r#type="number",
                    class="w-16 px-2 py-1 border border-gray-300 rounded-md text-center h-10",
                    placeholder="页码",
                    bind:value=jump_to_page,
                    // 增加回车键跳转
                    on:keypress=move |event: web_sys::KeyboardEvent| {
                        if event.key() == "Enter" {
                            // 调用跳转闭包
                            go_to_page(event);
                        }
                    }
                )
                // button(
                //     class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-semibold h-10",
                //     on:click=go_to_page
                // ) { "跳转" }
            }

            // NEW: 每页大小选择器
            div(class="flex items-center space-x-2 ml-4") {
                span(class="text-gray-700") { "每页：" }
                select(
                    class="px-2 py-1 border border-gray-300 rounded-md h-10",
                    on:change=change_page_size
                ) {
                    option(value="9", selected=page_size == 9) { "9" }
                    option(value="18", selected=page_size == 18) { "18" }
                    option(value="36", selected=page_size == 36) { "36" }
                    option(value="54", selected=page_size == 54) { "54" }
                    option(value="72", selected=page_size == 72) { "72" }
                }
            }
        }
    }
}