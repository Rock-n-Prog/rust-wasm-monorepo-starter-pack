//! Routes by yew_router

pub mod home;
pub mod videos;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use videos::Videos;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::Videos => html! {<Videos />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}