use stylist::yew::styled_component;
use yew::prelude::*;

use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

#[styled_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let theme_context = use_theme_context();

    let onclick = Callback::from(move |_| theme_context.switch_theme.emit(()));

    html! {
        <div class={css!(
            r#"
                padding: ${spacing_m} ${spacing_l};
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: ${spacing_m};
                text-align: center;

                @media only screen and (max-width: ${breakpoint_s}) {
                    justify-content: center;
                    flex-direction: column;
                }
            "#,
            spacing_m = theme_context.theme.spacings.m.clone(),
            spacing_l = theme_context.theme.spacings.l.clone(),
            breakpoint_s = theme_context.theme.breakpoints.s.clone(),
        )}>
            <h1>{ props.title.clone() }</h1>
            <button {onclick}>{ "Switch theme" }</button>
        </div>
    }
}
