use yew::prelude::*;
use stylist::yew::styled_component;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub text: String,
}

// TODO: Add and use Severity enum
#[styled_component(Alert)]
pub fn alert(props: &Props) -> Html {
    let theme_context = use_theme_context();

    html! {
        <div class={css!(
            r#"
                background-color: ${background};
                color: ${on_background};
                padding: ${spacing_s} ${spacing_m};
                border-radius: ${spacing_xxs};
                width: 100%;
            "#,
            background = theme_context.theme.colors.variants.error.background.clone(),
            on_background = theme_context.theme.colors.variants.error.on_background.clone(),
            spacing_xxs = theme_context.theme.spacings.xxs.clone(),
            spacing_s = theme_context.theme.spacings.s.clone(),
            spacing_m = theme_context.theme.spacings.m.clone(),
        )}>
            <p>{ props.text.clone() }</p>
        </div>
    }
}
