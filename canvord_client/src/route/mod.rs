use sycamore_router::Route;

#[derive(Route, Clone, PartialEq)]
pub enum AppRoutes {
    #[to("/")]
    Home,

    #[to("/article/<slug>")]
    Article { slug: String },

    #[not_found]
    NotFound,
}
