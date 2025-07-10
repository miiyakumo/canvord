mod component;
mod app;
mod view;
mod model;
mod api;
mod route;
mod layout;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}