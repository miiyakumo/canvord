use sycamore::prelude::*;

#[component]
pub fn Sidebar() -> View {
    view! {
        aside(class="w-64 h-screen bg-gray-800 text-white flex-shrink-0") {
            div(class="p-4 text-2xl font-bold") { "MyApp" }
            nav(class="mt-4") {
                ul {
                    li(class="px-4 py-2 hover:bg-gray-700 cursor-pointer") { "🏠 Dashboard" }
                    li(class="px-4 py-2 hover:bg-gray-700 cursor-pointer") { "📄 Articles" }
                    li(class="px-4 py-2 hover:bg-gray-700 cursor-pointer") { "📦 Inventory" }
                    li(class="px-4 py-2 hover:bg-gray-700 cursor-pointer") { "⚙ Settings" }
                }
            }
        }
    }
}