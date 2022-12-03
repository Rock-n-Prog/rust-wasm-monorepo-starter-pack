//! The root app contains initial authentication and url routes

use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::layouts::container::Container;
use crate::components::header::Header;
use crate::styles::global::GlobalStyles;
use crate::styles::theme::contexts::theme_context::ThemeProvider;
use super::routes::{switch, AppRoute};

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <ThemeProvider>
                <GlobalStyles />
                <Header title="ACME Web App" />
                <main>
                    <Container>
                        <Switch<AppRoute> render={switch} />
                    </Container>
                </main>
            </ThemeProvider>
        </BrowserRouter>
    }
}