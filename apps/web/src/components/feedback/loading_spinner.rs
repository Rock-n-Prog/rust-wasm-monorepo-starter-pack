use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

// Thanks to https://loading.io/css
#[styled_component(LoadingSpinner)]
pub fn loading_spinner() -> Html {
    let theme_context = use_theme_context();

    html! {
        <div class={css!(
            r#"
                display: inline-block;
                width: 100%;

                :after {
                    content: " ";
                    display: block;
                    width: 4rem;
                    height: 4rem;
                    margin: ${spacing_m} auto;
                    border-radius: 50%;
                    border: ${spacing_xxs} solid ${primary};
                    border-color: ${primary} transparent ${primary} transparent;
                    animation: loading-spinner 1.2s linear infinite;
                }

                @keyframes loading-spinner {
                  0% {
                    transform: rotate(0deg);
                  }
                  100% {
                    transform: rotate(360deg);
                  }
                }
            "#,
            spacing_xxs = theme_context.theme.spacings.xxs.clone(),
            spacing_m = theme_context.theme.spacings.m.clone(),
            primary = theme_context.theme.colors.palette.primary.main.clone(),
        )} />
    }
}
