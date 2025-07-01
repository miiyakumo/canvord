use sycamore::prelude::*;
use crate::layout::sidebar::Sidebar;

#[component(inline_props)]
pub fn Layout(children: Children) -> View {
    view! {
        div(class="flex h-screen") {
            Sidebar()
            div(class="flex-1 p-6 overflow-auto bg-gray-50") {
                (children)
            }
        }
    }
}
