use js_sys::Reflect;
use monaco::api::{CodeEditor, CodeEditorOptions};
use pulldown_cmark::Options;
use sycamore::prelude::*;
use sycamore::futures::spawn_local;
use sycamore::web::{create_client_resource, Suspense};
use wasm_bindgen::{JsCast, JsValue};
use crate::api::{get_article_by_id, update_article};
use crate::component::editor::Editor;
use crate::model::{Status, UpdateArticleCommand};
use crate::utils::{preview_html::preview_html, show_browser_notification::show_browser_notification};

#[component]
pub fn ArticleEditView(id: i64) -> View {
    // NEW: 使用 create_client_resource，就像 ArticleView 一样
    let article_resource = create_client_resource(move || async move {
        get_article_by_id(id).await
    });

    // 编辑器实例的 Signal
    let editor = create_signal(None::<CodeEditor>);

    // 表单字段的 Signal
    let title = create_signal(String::new());
    let slug = create_signal(String::new());
    let description = create_signal(String::new());
    let category = create_signal(String::new());
    let status = create_signal(Status::Unpublished);

    // 更新操作的加载状态
    let is_updating = create_signal(false);
    
    let resource_for_effect = article_resource.clone();
    // FIXED: 修正 create_effect 的用法
    // 这个 effect 的作用是：当文章数据成功加载后，用它来填充各个表单字段的 Signal
    create_effect(move || {
        // 只有当 resource 包含成功的数据时才执行
        if let Some(Ok(response)) = resource_for_effect.get_clone().as_ref() {
            if let Some(article) = &response.data {
                title.set(article.meta.title.clone());
                slug.set(article.meta.slug.clone());
                description.set(article.meta.description.clone());
                category.set(article.meta.category.clone());
                status.set(article.meta.status);

                editor.with(|editor_val| {
                    if let Some(ed) = editor_val {
                        if let Some(model) = ed.get_model() {
                            if model.get_value() != article.content_md {
                                model.set_value(&article.content_md);
                            }
                        }
                    }
                });
            }
        }
    });


    view! {
        div(class="flex flex-col h-screen") {
            // NEW: 仿照 ArticleView，使用 Suspense 来处理加载状态
            Suspense(fallback=|| view! { p(class="text-center text-gray-500 p-6") { "加载文章中..." } }) {
                // Suspense 的子节点会在 resource 加载完成后渲染
                (
                    // 使用 `if let` 来解包 resource 的值，比 match 更简洁
                    if let Some(Ok(response)) = article_resource.get_clone().as_ref() {
                        if let Some(article) = &response.data {
                            // NEW: 在数据加载成功后，才创建 Monaco 的 options
                            // 这样可以确保编辑器创建时就拥有了正确的初始内容
                            let opt = CodeEditorOptions::default()
                                .with_language("markdown".to_string())
                                .with_value(article.content_md.clone());

                            // 数据加载成功，渲染完整的编辑器UI
                            view! {
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
                                        div {
                                            select(
                                                id="status-select",
                                                class="border border-gray-300 rounded px-3 py-1 h-8",
                                                // 使用 on:change 来更新 status signal
                                                on:change=move |e: web_sys::Event| {
                                                    let value = e.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>().unwrap().value();
                                                    match value.as_str() {
                                                        "Published" => status.set(Status::Published),
                                                        "Unpublished" => status.set(Status::Unpublished),
                                                        "Hidden" => status.set(Status::Hidden),
                                                        _ => {}
                                                    }
                                                }
                                            ) {
                                                // 使用 `selected` 属性来同步 UI 和 Signal 的状态
                                                option(value="Published", selected=status.get() == Status::Published) { "已发布" }
                                                option(value="Unpublished", selected=status.get() == Status::Unpublished) { "未发布" }
                                                option(value="Hidden", selected=status.get() == Status::Hidden) { "已隐藏" }
                                            }
                                        }
                                        button(
                                            class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded font-semibold h-8 disabled:bg-gray-400",
                                            disabled=is_updating.get(),
                                            on:click=move |_| {
                                                is_updating.set(true);
                                                editor.with(|editor_val| {
                                                    if let Some(ed) = editor_val {
                                                        if let Some(model) = ed.get_model() {
                                                            let content_val = model.get_value();
                                                            let update_cmd = UpdateArticleCommand {
                                                                id,
                                                                slug: slug.get_clone(),
                                                                title: title.get_clone(),
                                                                description: description.get_clone(),
                                                                category: category.get_clone(),
                                                                status: status.get(),
                                                                content_md: content_val,
                                                            };
                                                            let title_for_notification = title.get_clone();
                                                            spawn_local(async move {
                                                                match update_article(&update_cmd).await {
                                                                    Ok(_) => {
                                                                        // 更新成功，显示浏览器通知
                                                                        show_browser_notification(
                                                                            "更新成功",
                                                                            &format!("文章《{}》已成功保存。", title_for_notification)
                                                                        ).await;
                                                                    }
                                                                    Err(err) => {
                                                                        // 更新失败，显示浏览器通知
                                                                        show_browser_notification(
                                                                            "更新失败",
                                                                            &format!("保存文章时发生错误: {}", err)
                                                                        ).await;
                                                                    }
                                                                }
                                                                is_updating.set(false);
                                                            });
                                                        }
                                                    }
                                                })
                                            }
                                        ) {
                                            (if is_updating.get() { "更新中..." } else { "更新" })
                                        }
                                        button(
                                            class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded font-semibold h-8",
                                            on:click=move |_| {
                                                editor.with(|opt| {
                                                    if let Some(ed) = opt {
                                                        if let Some(model) = ed.get_model() {
                                                            let content = model.get_value();
                                                            let parser = pulldown_cmark::Parser::new_ext(&*content, Options::all());
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
                                Editor(opt=opt.clone(), editor=editor)
                            }
                        } else {
                            // API 成功返回，但 data 字段为空
                            view! { p(class="text-center text-red-500 p-6") { (format!("未能找到 ID 为 {} 的文章。", id)) } }
                        }
                    } else if let Some(Err(e)) = article_resource.get_clone() {
                        let err = e.clone();
                        // API 请求失败
                        view! { p(class="text-center text-red-500 p-6") { "加载文章失败: " (err) } }
                    } else {
                        // 理论上这个分支不会被执行，因为 Suspense 已经处理了初始加载状态
                        view! { p { "..." } }
                    }
                )
            }
        }
    }
}