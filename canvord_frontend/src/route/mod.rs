use sycamore_router::Route;

#[derive(Route, Clone, PartialEq)]
pub enum AppRoutes {
    #[to("/")]
    Home,

    #[to("/article")]
    ArticleList,

    #[to("/article/edit/<id>")]
    ArticleEdit { id: i64 },

    #[to("/draft")]
    Draft,

    #[not_found]
    NotFound,
}
