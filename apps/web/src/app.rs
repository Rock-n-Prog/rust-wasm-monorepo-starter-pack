//! The root app contains initial authentication and url routes

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};
use crate::components::navbar::Navbar;
use crate::styles::global::GlobalStyles;
use crate::styles::theme::contexts::theme_context::ThemeProvider;

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <ThemeProvider>
                <GlobalStyles />
                <Navbar title="ACME Web App" />
                <main>
                    <Switch<AppRoute> render={switch} />
                </main>
            </ThemeProvider>
        </BrowserRouter>
    }
}