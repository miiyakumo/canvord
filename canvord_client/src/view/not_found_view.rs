use sycamore::{component, view};
use sycamore::prelude::View;

#[component]
pub fn NotFoundView() -> View {
    view!("404 Not Found")
}