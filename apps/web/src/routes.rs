use yew::prelude::*;
use yew_router::prelude::*;
use crate::domain::articles::components::article_page::ArticlePage;
use crate::domain::articles::components::articles_page::ArticlesPage;
use crate::domain::home::components::home_page::HomePage;
use crate::domain::not_found::components::not_found_page::NotFoundPage;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    #[at("/articles/:id")]
    Article { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<HomePage />},
        AppRoute::Articles => html! {<ArticlesPage />},
        AppRoute::Article { id } =>  html! {<ArticlePage id={id} />},
        AppRoute::NotFound => html! { <NotFoundPage /> },
    }
}