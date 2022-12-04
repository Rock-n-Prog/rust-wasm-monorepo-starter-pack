use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::components::typography::{heading_2::Heading2, body_1::Body1};

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <>
            <Heading2>{ "Not found" }</Heading2>
            <Link<AppRoute> to={AppRoute::Home}>
                <Body1>{ "Go home" }</Body1>
            </Link<AppRoute>>
        </>
    }
}