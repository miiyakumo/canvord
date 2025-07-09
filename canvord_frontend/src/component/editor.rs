use sycamore::prelude::*;
use monaco::api::*;
use sycamore::web::wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component(inline_props)]
pub fn Editor(opt: Option<CodeEditorOptions>, editor: Signal<Option<CodeEditor>>) -> View {
    let node_ref = create_node_ref();

    on_mount(move || {
        if let Some(html_element) = node_ref.get().dyn_into::<HtmlElement>().ok() {
            let edit = CodeEditor::create(&html_element, opt);
            editor.set(Some(edit));
        }
    });

    view! {
        div(ref=node_ref, class="flex-1", style="height: calc(100vh - 3rem);")
    }
}