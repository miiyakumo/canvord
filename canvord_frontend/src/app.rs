use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};
use crate::layout::layout::Layout;
use crate::route::AppRoutes;
use crate::view::edit_view::ArticleEditView;
use crate::view::article_view::ArticleView;
use crate::view::draft_view::DraftView;
use crate::view::home_view::HomeView;
use crate::view::not_found_view::NotFoundView;

#[component]
pub fn App() -> View {
    view! {
        Layout {
            Router(
                integration = HistoryIntegration::new(),
                view = |route: ReadSignal<AppRoutes>| {
                    view! {
                        div {
                            (match route.get_clone() {
                                AppRoutes::Home => HomeView(),
                                AppRoutes::ArticleList => ArticleView(),
                                AppRoutes::Draft => DraftView(),
                                AppRoutes::ArticleEdit { id } => ArticleEditView(id),
                                AppRoutes::NotFound => NotFoundView(),
                            })
                        }
                    }
                }
            )
        }
    }
}