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
use crate::utils::preview_html::preview_html;
use crate::utils::show_browser_notification::show_browser_notification;

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
                                        let title_for_notification = title.get_clone();
                                        spawn_local(async move {
                                            match create_article(&cmd).await {
                                                Ok(resp) => {
                                                    match resp.code {
                                                        0 => {
                                                            show_browser_notification(
                                                                "发布成功",
                                                                &format!("文章《{}》已成功保存。", title_for_notification)
                                                            ).await;
                                                            // 可选：弹窗提示、跳转、清空表单等
                                                            if let Some(window) = web_sys::window() {
                                                                if let Ok(Some(storage)) = window.local_storage() {
                                                                    let _ = storage.remove_item(LOCAL_DRAFT_KEY);
                                                                    model.set_value("");
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            show_browser_notification(
                                                                "发布失败",
                                                                &format!("{}", resp.message)
                                                            ).await;
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    // 更新失败，显示浏览器通知
                                                    show_browser_notification(
                                                        "发布失败",
                                                        &format!("保存文章时发生错误: {}", err)
                                                    ).await;
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