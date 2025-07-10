use crate::app::App;

mod app;
mod model;
mod route;
mod api;
mod view;

fn main() {
    sycamore::render(App);
}
