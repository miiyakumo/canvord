use sycamore::prelude::*;

#[component(inline_props)]
pub fn TextInput(value: Signal<String>, placeholder: &'static str) -> View {
    view! {
        input(
            r#type="text",
            bind:value=value,
            placeholder=placeholder,
            class="px-2 py-1 border rounded"
        )
    }
}
