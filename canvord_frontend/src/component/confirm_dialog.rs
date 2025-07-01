use sycamore::prelude::*;

#[component(inline_props)]
pub fn ConfirmDialog(
    title: &'static str,
    message: &'static str,
    on_confirm: Box<dyn Fn()>,
    on_cancel: Box<dyn Fn()>,
) -> View {
    view! {
        div(class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50") {
            div(class="bg-white rounded-md p-6 max-w-sm w-full shadow-lg") {
                h3(class="text-lg font-semibold mb-4") { (title) }
                p(class="mb-6") { (message) }
                div(class="flex justify-end space-x-4") {
                    button(
                        class="px-4 py-2 rounded bg-gray-300 hover:bg-gray-400",
                        on:click=move |_| on_cancel()
                    ) { "取消" }
                    button(
                        class="px-4 py-2 rounded bg-red-600 text-white hover:bg-red-700",
                        on:click=move |_| on_confirm()
                    ) { "确认" }
                }
            }
        }
    }
}
