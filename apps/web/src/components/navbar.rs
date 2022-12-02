use stylist::yew::styled_component;
use yew::prelude::*;

use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

// TODO: Use theme values
#[styled_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    // TODO: Getting "the trait `Hook` is not implemented for `ThemeContext`"
    let theme_context = use_theme_context();

    html! {
        <div class={css!(r#"
            padding: {spacing_xs} {spacing_m};
            display: flex;
            justify-content: center;

            @media only screen and (max-width: 767px) {
                justify-content: flex-start;
            }
        "#,
            spacing_xs = theme_context.theme.spacings.xs.clone(),
            spacing_m = theme_context.theme.spacings.m.clone(),
        )}>
            <h1>{ props.title.clone() }</h1>
        </div>
    }
}
