use monaco::api::{CodeEditor, CodeEditorOptions};
use crate::component::editor::Editor;
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use js_sys::Reflect;
use gloo_timers::callback::Interval;
use sycamore::futures::spawn_local;
use crate::api::create_article;
use crate::model::CreateArticleCommand;

#[component]
pub fn DraftView() -> View {
    const LOCAL_DRAFT_KEY: &str = "local_draft";

    let initial_content = web_sys::window()
        .and_then(|win| win.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(LOCAL_DRAFT_KEY).ok().flatten())
        .unwrap_or_else(|| "".to_string());

    let opt = CodeEditorOptions::default()
        .with_language("markdown".to_string())
        .with_value(initial_content);

    let editor = create_signal(None::<CodeEditor>);

    // 额外字段 Signal
    let title = create_signal(String::new());
    let slug = create_signal(String::new());
    let description = create_signal(String::new());
    let category = create_signal(String::new());

    let auto_save_interval = create_signal(None::<Interval>);
    
    create_effect(move || {
        let new_interval = Interval::new(5000, move || {
            editor.with(|opt| {
                if let Some(ed) = opt {
                    if let Some(model) = ed.get_model() {
                        if let Some(win) = web_sys::window() {
                            if let Ok(Some(storage)) = win.local_storage() {
                                let _ = storage.set_item(LOCAL_DRAFT_KEY, &model.get_value());
                            }
                        }
                    }
                }
            })
        });
        auto_save_interval.set(Some(new_interval));
    });
    
    // 初始化时尝试恢复内容
    let _restore_on_mount = create_effect(move || {
        editor.with(|opt| {
            if let Some(ed) = opt {
                if let Some(model) = ed.get_model() {
                    if let Some(win) = web_sys::window() {
                        if let Ok(Some(storage)) = win.local_storage() {
                            if let Ok(Some(saved)) = storage.get_item(LOCAL_DRAFT_KEY) {
                                model.set_value(&saved);
                            }
                        }
                    }
                }
            }
        })
    });

    view! {
        div(class="flex flex-col h-screen") {
            // 顶栏
            div(class="bg-white shadow p-4 flex flex-col space-y-2") {
                div(class="flex flex-wrap gap-4 items-center") {
                    input(
                        class="border border-gray-300 rounded px-3 py-1 w-48 h-8",
                        placeholder="Slug",
                        bind:value=slug
                    )
                    input(
                        class="border border-gray-300 rounded px-3 py-1 w-64 h-8",
                        placeholder="Title",
                        bind:value=title
                    )
                    input(
                        class="border border-gray-300 rounded px-3 py-1 w-64 h-8",
                        placeholder="Category",
                        bind:value=category
                    )
                    textarea(
                        class="border border-gray-300 rounded px-3 py-1 w-96 h-8",
                        placeholder="Description",
                        bind:value=description
                    )
                    button(
                        class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded font-semibold h-8",
                        on:click=move |_| {
                            let slug_val = slug.get_clone();
                            let title_val = title.get_clone();
                            let desc_val = description.get_clone();
                            let category_val = category.get_clone();

                            editor.with(|editor_val| {
                                if let Some(ed) = editor_val {
                                    if let Some(model) = ed.get_model() {
                                        let content_val = model.get_value();

                                        let cmd = CreateArticleCommand {
                                            slug: slug_val,
                                            title: title_val,
                                            description: desc_val,
                                            category: category_val,
                                            content_md: content_val,
                                        };

                                        // 异步调用发布接口
                                        spawn_local(async move {
                                            match create_article(&cmd).await {
                                                Ok(resp) => {
                                                    web_sys::console::log_1(&format!("发布成功: {:?}", resp.data).into());
                                                    // 可选：弹窗提示、跳转、清空表单等
                                                    if let Some(window) = web_sys::window() {
                                                        if let Ok(Some(storage)) = window.local_storage() {
                                                            let _ = storage.remove_item(LOCAL_DRAFT_KEY);
                                                            model.set_value("");
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    web_sys::console::log_1(&format!("发布失败: {}", err).into());
                                                }
                                            }
                                        });
                                    }
                                }
                            })
                        }
                    ) {
                        "发布"
                    }
                    button(
                        class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded font-semibold h-8",
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
                                                    let html = preview_html(html_output);

                                                    let _ = Reflect::get(&doc, &JsValue::from_str("write"))
                                                        .and_then(|f| {
                                                            if let Some(func) = f.dyn_ref::<js_sys::Function>() {
                                                                func.call1(&doc, &JsValue::from_str(&html))
                                                            } else {
                                                                Err(JsValue::from_str("write is not a function"))
                                                            }
                                                        });

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
                        "预览"
                    }
                }
            }

            // 编辑器主体
            Editor(opt=opt, editor=editor)
        }
    }
}

fn preview_html(_html_output: String) -> String {
    format!(r#"<!DOCTYPE html>
               <html lang="cn">
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
               </html>"#, _html_output)
}
