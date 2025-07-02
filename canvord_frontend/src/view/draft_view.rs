use monaco::api::{CodeEditor, CodeEditorOptions};
use crate::component::editor;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;

#[component]
pub fn DraftView() -> View {
    // 定义模式选择状态：0 - 纯MD编辑，1 - 双屏实时预览
    let mode = create_signal(0);

    // Markdown 编辑内容
    let content = create_signal("".to_string());

    // Monaco 编辑器实例
    let editor = create_signal(None::<CodeEditor>);

    // 编辑器配置
    let opt = create_signal(CodeEditorOptions::default()
        .with_language("markdown".to_string())
    );

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
        // 这里可以插入 Markdown 到 HTML 的解析逻辑
        format!("<div>{}</div>", md_content)
    };

    view! {
        div(class="relative h-full w-full overflow-hidden") {
            // 主内容区域
            div(class="flex w-full h-full") {
                // 根据当前模式显示不同的内容
                (match mode.get() {
                    0 => view!{
                        div(class="w-full h-full") {
                            editor::Editor(
                                opt=opt.get_clone(),
                                editor=editor
                            )
                        }
                    },
                    1 => view!{
                        div(class="flex w-full h-full space-x-4") {
                            // 编辑器
                            div(class="w-1/2 h-full") {
                                editor::Editor(
                                    opt=opt.get_clone(),
                                    editor=editor
                                )
                            }
                            // 预览
                            div(class="w-1/2 h-full p-4 border-2 border-gray-300 rounded-md overflow-auto") {
                                (preview_content()) // 使用编辑内容渲染预览
                            }
                        }
                    },
                    _ => view!{}
                })
            }

            // 底部按钮区域，固定在页面底部
            div(class="fixed bottom-0 left-0 right-0 bg-blue-600 text-white p-2 flex justify-between items-center w-full z-10 shadow-md") {
                // 切换按钮，放在底部中间
                button(on:click=move |_| toggle_mode(if mode.get() == 0 { 1 } else { 0 }),
                    class="p-1 bg-blue-500 text-white rounded-full hover:bg-blue-600 transition duration-300"
                ) {
                    "切换模式"
                }
                button(on:click=move |_| toggle_mode(if mode.get() == 0 { 1 } else { 0 }),
                    class="p-1 bg-blue-500 text-white rounded-full hover:bg-blue-600 transition duration-300"
                ) {
                    "保存文章"
                }
            }
        }
    }
}
