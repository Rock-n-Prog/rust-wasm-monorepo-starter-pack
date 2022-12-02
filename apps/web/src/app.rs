//! The root app contains initial authentication and url routes

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};
use crate::styles::global::GlobalStyles;

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <GlobalStyles />
            <h1>{ "ACME Web App" }</h1>
            <main>
                <Switch<AppRoute> render={switch} />
            </main>
        </BrowserRouter>
    }
}