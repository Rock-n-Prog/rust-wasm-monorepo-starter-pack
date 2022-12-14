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
                    font-family: ${sans_serif};
                    font-weight: ${font_weight_regular};
                    font-size: ${font_size_m};
                }

                h1, h2, h3, p, span, a, li {
                    opacity: ${font_emphasis_high};
                }

                a {
                    color: inherit;
                }

                a:hover {
                    color: ${primary};
                }
            "#,
            sans_serif = theme_context.theme.fonts.families.sans_serif,
            font_weight_regular = theme_context.theme.fonts.weights.regular,
            font_size_m = theme_context.theme.fonts.sizes.m,
            font_emphasis_high = theme_context.theme.fonts.emphasis.high,
            background = theme_context.theme.colors.background,
            on_background = theme_context.theme.colors.on_background,
            primary = theme_context.theme.colors.palette.primary.main,
        )} />
    }
}