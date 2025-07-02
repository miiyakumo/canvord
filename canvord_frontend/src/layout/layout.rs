use sycamore::prelude::*;
use crate::layout::sidebar::Sidebar;

#[component(inline_props)]
pub fn Layout(children: Children) -> View {
    view! {
        div(class="flex h-screen items-stretch") {
            Sidebar()
            div(class="flex-1 h-screen overflow-hidden bg-gray-50") {
                (children)
            }
        }
    }
}