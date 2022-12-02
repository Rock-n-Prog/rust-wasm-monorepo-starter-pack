//! Routes by yew_router

use yew::prelude::*;
use yew_router::prelude::*;

use crate::domain::home::components::home_page::HomePage;
use crate::domain::articles::components::articles_page::ArticlesPage;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<HomePage />},
        AppRoute::Articles => html! {<ArticlesPage />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}