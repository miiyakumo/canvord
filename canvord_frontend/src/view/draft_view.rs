use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;

#[component]
pub fn DraftView() -> View {
    // 定义模式选择状态：0 - 纯MD编辑，1 - 双屏实时预览
    let mode = create_signal(0);

    // Markdown 编辑内容
    let content = create_signal("".to_string());

    // 更新文章内容
    let on_input = move |event: web_sys::Event| {
        let input = event.target().unwrap().dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
        content.set(input.value());
    };

    // 切换模式函数
    let toggle_mode = move |new_mode: usize| {
        mode.set(new_mode);
    };

    // 双屏实时预览
    let preview_content = move || {
        let md_content = content.get_clone();
        // 使用一个Markdown解析器，例如 `marked` 解析md内容到HTML
        // marked::marked(md_content)
        "md"
    };

    view! {
        div(class="relative h-screen w-full") {
            // 切换按钮，放在右上角
            button(on:click=move |_| toggle_mode(if mode.get() == 0 { 1 } else { 0 }),
                class="absolute top-4 right-4 p-2 bg-blue-500 text-white rounded-full hover:bg-blue-600"
            ) {
                "切换模式"
            }

            // 根据当前模式显示不同的内容
            (match mode.get() {
                0 => view!{
                    div(class="w-full h-full") {
                        textarea(
                            bind:value=content,
                            on:input=on_input,
                            placeholder="请输入Markdown内容...",
                            class="w-full h-full p-4 border-2 border-gray-300 rounded-md resize-none",
                            rows="20"
                        )
                    }
                },
                1 => view!{
                    div(class="flex w-full h-full") {
                        // 编辑器
                        div(class="w-1/2 pr-4") {
                            textarea(
                                bind:value=content,
                                on:input=on_input,
                                placeholder="请输入Markdown内容...",
                                class="w-full h-full p-4 border-2 border-gray-300 rounded-md resize-none",
                                rows="20"
                            )
                        }
                        // 预览
                        div(class="w-1/2 pl-4") {
                            div(class="w-full h-full p-4 border-2 border-gray-300 rounded-md overflow-auto") {
                                (preview_content())
                            }
                        }
                    }
                },
                _ => view!{}
            })
        }
    }
}
