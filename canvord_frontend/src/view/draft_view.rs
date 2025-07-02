use monaco::api::CodeEditorOptions;
use crate::component::editor;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;

#[component]
pub fn DraftView() -> View {
    // 定义模式选择状态：0 - 纯MD编辑，1 - 双屏实时预览
    let mode = create_signal(0);

    // Markdown 编辑内容
    let content = create_signal("".to_string());

    let opt = create_signal(CodeEditorOptions::default()
        .with_language("markdown".to_string()));

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
        "md" // 这里可以插入 Markdown 到 HTML 的解析逻辑
    };

    view! {
        div(class="relative h-screen w-full overflow-hidden") {
            // 顶栏
            // 主内容区域
           div(class="flex w-full h-full") {  // 移除 pt-12，直接填满剩余空间
                // 根据当前模式显示不同的内容
                (match mode.get() {
                    0 => view!{
                        div(class="w-full h-full") {
                            editor::Editor(
                                opt=opt.get_clone(),
                            )
                        }
                    },
                    1 => view!{
                        div(class="flex w-full h-full space-x-4") {  // 加上间距，使左右两侧内容不挤在一起
                            // 编辑器
                            div(class="w-1/2 h-full") {
                                editor::Editor(
                                    opt=opt.get_clone(),
                                )
                            }
                            // 预览
                            div(class="w-1/2 h-full p-4 border-2 border-gray-300 rounded-md overflow-auto") {
                                (preview_content())
                            }
                        }
                    },
                    _ => view!{}
                })
            }
        }
    }
}

