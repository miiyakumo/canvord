use sycamore::prelude::*;

#[component(inline_props)]
pub fn TextInput(value: Signal<String>) -> View {
    view! {
        input(
            r#type="text",
            bind:value=value
        )
    }
}
