use sycamore::prelude::*;
use monaco::api::*;
use sycamore::web::wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component(inline_props)]
pub fn Editor(opt: Option<CodeEditorOptions>, editor: Signal<Option<CodeEditor>>) -> View {
    // 创建 NodeRef
    let node_ref = create_node_ref();
    // 在组件挂载后，获取 DOM 元素并初始化编辑器
    on_mount(move || {
        // 获取原生 DOM 元素的引用
        let node = node_ref.get();

        // 确保获取到有效的 node，并将其强制转换为 HtmlElement
        if let Some(html_element) = node.dyn_into::<HtmlElement>().ok() {
            // 成功转换后，将 HtmlElement 传递给 CodeEditor::create
            let edit = CodeEditor::create(&html_element, opt);
            // 进一步操作...
            editor.set(Some(edit));
        } else {
            console_log!("Failed to cast to HtmlElement");
        }
    });
    // BUG: 这里疑似会导致布局问题，比如被顶栏或底栏覆盖一部分内容
    view! {
        div(r#ref=node_ref, style="height: 100vh;")
    }
}