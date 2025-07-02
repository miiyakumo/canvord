use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn Sidebar() -> View {
    view! {
        nav(class="w-64 bg-white shadow-md h-full p-4 space-y-2") {
            h2(class="text-xl font-bold mb-4") { "Canvord" }
            SidebarItem(label = "🏠 主页", route = "/")
            SidebarItem(label = "📝 文章", route = "/article")
            SidebarItem(label = "✍ 撰写", route = "/draft")
        }
    }
}

#[component(inline_props)]
fn SidebarItem(label: &'static str, route: &'static str) -> View {
    view! {
        button(
            class="block w-full text-left text-gray-700 hover:bg-gray-100 px-3 py-2 rounded-md transition",
            on:click=move |_| navigate(route)
        ) {
            (label)
        }
    }
}