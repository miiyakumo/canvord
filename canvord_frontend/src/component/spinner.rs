use sycamore::prelude::*;

#[component]
pub fn Spinner() -> View {
    view! {
        div(class="animate-spin rounded-full h-5 w-5 border-b-2 border-white") {}
    }
}
