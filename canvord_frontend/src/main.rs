mod component;
mod app;
mod view;
mod model;
mod api;
mod route;
mod layout;
mod utils;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}