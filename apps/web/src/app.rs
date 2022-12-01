//! The root app contains initial authentication and url routes

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <h1>{ "ACME Web App" }</h1>
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}