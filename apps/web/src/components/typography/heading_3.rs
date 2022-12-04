use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(Heading3)]
pub fn heading_3(props: &Props) -> Html {
    let theme_context = use_theme_context();

    html! {
        <h3 class={css!(
            r#"
                font-size: ${font_size_l};
                margin-bottom: ${spacing_xs};
            "#,
            font_size_l = theme_context.theme.fonts.sizes.l.clone(),
            spacing_xs = theme_context.theme.spacings.xs.clone(),
        )}>
            { props.children.clone() }
        </h3>
    }
}
