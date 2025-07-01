mod component;
mod app;
mod view;
mod model;
mod api;
mod route;
mod layout;

use crate::app::App;

fn main() {
    sycamore::render(App);
}