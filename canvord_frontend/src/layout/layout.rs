use sycamore::prelude::*;
use crate::layout::sidebar::Sidebar;

#[component(inline_props)]
pub fn Layout(children: Children) -> View {
    view! {
        div(class="min-h-screen flex") {
            Sidebar()
            div(class="flex-1 bg-gray-50 overflow-y-auto p-6") {
                (children)
            }
        }
    }
}