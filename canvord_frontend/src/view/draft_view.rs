use monaco::api::{CodeEditor, CodeEditorOptions};
use crate::component::editor::Editor;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use js_sys::Reflect;

#[component]
pub fn DraftView() -> View {
    let opt = CodeEditorOptions::default()
        .with_language("markdown".to_string())
        .with_value("Hello Markdown!".to_string());

    let editor = create_signal(None::<CodeEditor>);

    view! {
        div(class="flex flex-col h-screen") {
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
                                                let html = format!(r#"<!DOCTYPE html>
                                                                            <html lang="en">
                                                                            <head>
                                                                                <meta charset="utf-8">
                                                                                <title>Markdown Preview</title>
                                                                                <meta name="viewport" content="width=device-width, initial-scale=1">
                                                                                <style>
                                                                                    body {{
                                                                                        max-width: 768px;
                                                                                        margin: 2rem auto;
                                                                                        padding: 2rem;
                                                                                        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
                                                                                        line-height: 1.6;
                                                                                        color: #2e2e2e;
                                                                                        background-color: #fafafa;
                                                                                    }}
                                                                                    h1, h2, h3 {{
                                                                                        border-bottom: 1px solid #eaecef;
                                                                                        padding-bottom: 0.3em;
                                                                                        margin-top: 1.5em;
                                                                                    }}
                                                                                    pre, code {{
                                                                                        background-color: #f6f8fa;
                                                                                        padding: 0.2em 0.4em;
                                                                                        border-radius: 6px;
                                                                                        font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
                                                                                    }}
                                                                                    pre {{
                                                                                        padding: 1em;
                                                                                        overflow: auto;
                                                                                    }}
                                                                                    blockquote {{
                                                                                        color: #6a737d;
                                                                                        padding: 0 1em;
                                                                                        border-left: 0.25em solid #dfe2e5;
                                                                                    }}
                                                                                    ul {{
                                                                                        list-style: disc;
                                                                                        margin-left: 2em;
                                                                                    }}
                                                                                    table {{
                                                                                        border-collapse: collapse;
                                                                                    }}
                                                                                    th, td {{
                                                                                        border: 1px solid #dfe2e5;
                                                                                        padding: 6px 13px;
                                                                                    }}
                                                                                    img {{
                                                                                        max-width: 100%;
                                                                                    }}
                                                                                    @media (prefers-color-scheme: dark) {{
                                                                                        body {{
                                                                                            color: #d1d5db;
                                                                                            background-color: #1f2937;
                                                                                        }}
                                                                                        a {{ color: #93c5fd; }}
                                                                                        code, pre {{
                                                                                            background-color: #374151;
                                                                                            color: #f3f4f6;
                                                                                        }}
                                                                                    }}
                                                                                </style>
                                                                            </head>
                                                                            <body>
                                                                                {}
                                                                            </body>
                                                                            </html>"#, html_output);

                                                // 用 JS 调用 document.write()
                                                let _ = Reflect::get(&doc, &JsValue::from_str("write"))
                                                    .and_then(|f| {
                                                        if let Some(func) = f.dyn_ref::<js_sys::Function>() {
                                                            func.call1(&doc, &JsValue::from_str(&html))
                                                        } else {
                                                            Err(JsValue::from_str("write is not a function"))
                                                        }
                                                    });

                                                // 关闭文档
                                                let _ = Reflect::get(&doc, &JsValue::from_str("close"))
                                                    .and_then(|f| {
                                                        if let Some(func) = f.dyn_ref::<js_sys::Function>() {
                                                            func.call0(&doc)
                                                        } else {
                                                            Err(JsValue::from_str("close is not a function"))
                                                        }
                                                    });
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

            // 编辑器
            Editor(opt=opt, editor=editor)
        }
    }
}
