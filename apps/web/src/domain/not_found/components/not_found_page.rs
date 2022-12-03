use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <>
            <h2>{ "Not found" }</h2>
            <Link<AppRoute> to={AppRoute::Home}>
                { "Go home" }
            </Link<AppRoute>>
        </>
    }
}