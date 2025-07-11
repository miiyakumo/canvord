use crate::app::App;

mod app;
mod model;
mod route;
mod api;
mod view;
mod component;

fn main() {
    sycamore::render(App);
}
