mod component;
mod app;
mod view;

use crate::app::App;

fn main() {
    sycamore::render(App);
}