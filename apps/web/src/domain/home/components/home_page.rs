use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <p>{ "Home" }</p>
            <Link<AppRoute> to={AppRoute::Articles}>
                { "Articles" }
            </Link<AppRoute>>
        </>
    }
}