use sycamore::prelude::*;

#[component(inline_props)]
pub fn Button(label: &'static str, disabled: bool, on_click: Box<dyn Fn()>) -> View {
    view! {
        button(
            class=format!(
                "inline-flex justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 {}",
                if disabled { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" }
            ),
            disabled=disabled,
            on:click=move |_| on_click()
        ) {
            (label)
        }
    }
}
