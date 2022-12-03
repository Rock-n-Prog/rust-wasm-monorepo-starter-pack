use stylist::yew::styled_component;
use yew::prelude::*;

use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(Container)]
pub fn container(props: &Props) -> Html {
    let theme_context = use_theme_context();

    html! {
        <div class={css!(
            r#"
                padding: 0 ${spacing_l};
                margin: ${spacing_l} auto 0;
                max-width: ${container_max_width};

                @media only screen and (max-width: ${breakpoint_s}) {
                    max-width: unset;
                }
            "#,
            spacing_l = theme_context.theme.spacings.l.clone(),
            container_max_width = theme_context.theme.spacings.container_max_width.clone(),
            breakpoint_s = theme_context.theme.breakpoints.s.clone(),
        )}>
            { props.children.clone() }
        </div>
    }
}
