use monaco::api::{CodeEditor, CodeEditorOptions};
use crate::component::editor::Editor;
use sycamore::prelude::*;

#[component]
pub fn DraftView() -> View {
    // 编辑器配置项
    let opt = CodeEditorOptions::default()
            .with_language("markdown".to_string())
            .with_value("Hello Markdown!".to_string());

    // 提取 editor Signal 到上层作用域，便于按钮访问
    let editor = create_signal(None::<CodeEditor>);

    view! {
        div(class="flex flex-col h-screen") {
            // 顶栏，按钮在这里
            div(class="h-12 bg-gray-800 text-white flex items-center px-4 space-x-4") {
                button(
                    class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-1 rounded",
                    on:click=move |_| {
                        editor.with(|opt| {
                            if let Some(ed) = opt {
                                if let Some(model) = ed.get_model() {
                                    let content = model.get_value();
                                    let parser = pulldown_cmark::Parser::new(&*content);
                                    let mut html_output = String::new();
                                    pulldown_cmark::html::push_html(&mut html_output, parser);
                                    
                                    if let Some(win) = web_sys::window() {
                                        if let Ok(Some(new_tab)) = win.open_with_url("about:blank") {
                                            if let Some(doc) = new_tab.document() {
                                                if let Some(body) = doc.body() {
                                                    body.set_inner_html(&html_output);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }
                ) {
                    "Show Editor Content"
                }
            }

            // 编辑器本体
            Editor(opt=opt, editor=editor)
        }
    }
}
