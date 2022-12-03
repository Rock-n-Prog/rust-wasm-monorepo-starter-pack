use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <h2>{ "Home" }</h2>
            <Link<AppRoute> to={AppRoute::Articles}>
                { "Articles" }
            </Link<AppRoute>>
        </>
    }
}