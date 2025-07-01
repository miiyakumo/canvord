use sycamore::prelude::*;
use crate::component::sidebar::Sidebar;

#[component]
pub fn App() -> View {
    view! {
        div(class="flex h-screen") {
            Sidebar()
            main(class="flex-1 bg-gray-100 p-6 overflow-auto") {
                h1(class="text-3xl font-bold mb-4") { "欢迎使用 Sycamore + Tailwind!" }
                p { "这里是主内容区。" }
            }
        }
    }
}