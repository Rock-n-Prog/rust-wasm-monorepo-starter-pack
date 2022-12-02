//! The root app contains initial authentication and url routes

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};
use crate::components::navbar::Navbar;
use crate::styles::global::GlobalStyles;

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <GlobalStyles />
            <Navbar title="ACME Web App" />
            <main>
                <Switch<AppRoute> render={switch} />
            </main>
        </BrowserRouter>
    }
}