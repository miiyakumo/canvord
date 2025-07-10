use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};
use crate::route::AppRoutes;
use crate::view::*;

#[component]
pub fn App() -> View {
    view! {
        Router(
            integration = HistoryIntegration::new(),
            view = |route: ReadSignal<AppRoutes>| {
                view! {
                    div {
                        (match route.get_clone() {
                            AppRoutes::Home => HomeView(),
                            AppRoutes::Article { slug } => ArticleView(slug),
                            AppRoutes::NotFound => NotFoundView(),
                        })
                    }
                }
            }
        )
    }
}