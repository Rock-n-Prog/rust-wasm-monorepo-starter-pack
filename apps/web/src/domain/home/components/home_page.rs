use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::components::typography::{heading_2::Heading2, body_1::Body1};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <Heading2>{ "Home" }</Heading2>
            <Link<AppRoute> to={AppRoute::Articles}>
                <Body1>{ "Articles" }</Body1>
            </Link<AppRoute>>
        </>
    }
}