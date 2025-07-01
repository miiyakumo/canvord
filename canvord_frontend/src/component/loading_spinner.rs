use sycamore::prelude::*;

#[component]
pub fn LoadingSpinner() -> View {
    view! {
        div(class="flex justify-center items-center p-4") {
            svg(class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-500", xmlns="http://www.w3.org/2000/svg", fill="none", viewBox="0 0 24 24") {
                circle(class="opacity-25", cx="12", cy="12", r="10", stroke="currentColor", stroke-width="4") {}
                path(class="opacity-75", fill="currentColor", d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z") {}
            }
        }
    }
}