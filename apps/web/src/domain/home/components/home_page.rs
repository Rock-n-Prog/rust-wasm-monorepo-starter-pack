use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::components::typography::heading_2::Heading2;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <Heading2>{ "Home" }</Heading2>
            <Link<AppRoute> to={AppRoute::Articles}>
                { "Articles" }
            </Link<AppRoute>>
        </>
    }
}