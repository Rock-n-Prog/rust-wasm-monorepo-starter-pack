use stylist::yew::{styled_component, Global};
use yew::prelude::*;

use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[styled_component(GlobalStyles)]
pub fn global_styles() -> Html {
    let theme_context = use_theme_context();

    html! {
        <Global css={css!(
            r#"
                * {
                    padding: 0;
                    margin: 0;
                    height: min-content;
                }

                html, body {
                    font-family: sans-serif;
                    min-height: 100vh;
                    background-color: ${background};
                    color: ${on_background};
                }
            "#,
            background = theme_context.theme.colors.background,
            on_background = theme_context.theme.colors.on_background,
        )} />
    }
}