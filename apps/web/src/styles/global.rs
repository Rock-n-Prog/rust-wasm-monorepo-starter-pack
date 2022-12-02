use stylist::yew::{styled_component, Global};
use yew::prelude::*;

use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[styled_component(GlobalStyles)]
pub fn global_styles() -> Html {
    let theme_context = use_theme_context();

    html! {
        <Global css={css!(
            r#"
                html, body {
                    font-family: sans-serif;
                    padding: 0;
                    margin: 0;
                    min-height: 100vh;
                    background-color: ${background};
                }
            "#,
            background = theme_context.theme.colors.background,
        )} />
    }
}