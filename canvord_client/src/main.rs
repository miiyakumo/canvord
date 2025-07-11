use crate::app::App;

mod app;
mod model;
mod route;
mod api;
mod view;
mod component;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
