use js_sys::{Array, Object, Reflect, RegExp};
use sycamore::prelude::*;
use monaco::api::*;
use monaco::sys::languages::{set_language_configuration, IndentAction};
use sycamore::web::wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

#[component(inline_props)]
pub fn Editor(opt: Option<CodeEditorOptions>, editor: Signal<Option<CodeEditor>>) -> View {
    let node_ref = create_node_ref();

    on_mount(move || {
        if let Some(html_element) = node_ref.get().dyn_into::<HtmlElement>().ok() {
            let edit = CodeEditor::create(&html_element, opt);
            editor.set(Some(edit));
        }
        register_enter_rule_for_markdown();
    });

    view! {
        div(ref=node_ref, class="flex-1", style="height: calc(100vh - 3rem);")
    }
}

fn register_enter_rule_for_markdown() {
    fn make_rule(
        pattern: &str,
        append: &str,
        remove_text: Option<u32>,
        indent_action: Option<IndentAction>,
    ) -> Object {
        let rule = Object::new();
        let re = RegExp::new(pattern, "");
        let _ = Reflect::set(&rule, &JsValue::from_str("beforeText"), &re);

        let action = Object::new();
        let final_indent_action = indent_action.unwrap_or(IndentAction::None);
        let _ = Reflect::set(
            &action,
            &JsValue::from_str("indentAction"),
            &JsValue::from(final_indent_action as u32),
        );

        if !append.is_empty() {
            let _ = Reflect::set(&action, &JsValue::from_str("appendText"), &JsValue::from_str(append));
        }

        if let Some(n) = remove_text {
            let _ = Reflect::set(&action, &JsValue::from_str("removeText"), &JsValue::from(n));
        }

        let _ = Reflect::set(&rule, &JsValue::from_str("action"), &action);
        rule
    }

    let rules = Array::new();

    let patterns: Vec<(&str, &str, Option<u32>, Option<IndentAction>)> = vec![
        // `-` 列表
        (r"^\s*-\s+.+", "- ", None, None),
        // `*` 列表
        (r"^\s*\*\s+.+", "* ", None, None),
        // `+` 列表
        (r"^\s*\+\s+.+", "+ ", None, None),
        // 有序列表。
        (r"^\s*(\d+\.)\s+.+", "1. ", None, None),
        // 续写任务列表
        (r"^\s*-\s\[[xX\s]\]\s+.+", "- [ ] ", None, None),
        // 续写引用块
        (r"^\s*>\s+.+", "> ", None, None),
        // 续写表格行
        (r"^\s*\|.+(?:\|)?\s*$", "| ", None, None),

        // --- 退出规则 ---

        // 退出无序、有序和引用列表
        (r"^\s*(([*\-+]|\d+\.)\s*|>\s*)$", "", None, Some(IndentAction::Outdent)),

        // 退出任务列表
        (r"^\s*-\s\[[xX\s]\]\s*$", "", None, Some(IndentAction::Outdent)),

        // --- 特殊规则 ---

        // 取消创建代码块
        (r"^\s*```\s*$", "", Some(3), None),
    ];

    for (pattern, append, remove, indent) in patterns {
        rules.push(&make_rule(pattern, append, remove, indent));
    }

    let config = Object::new();
    let _ = Reflect::set(&config, &JsValue::from_str("onEnterRules"), &rules);

    // Apply the configuration to the 'markdown' language
    set_language_configuration("markdown", config.unchecked_ref());
}