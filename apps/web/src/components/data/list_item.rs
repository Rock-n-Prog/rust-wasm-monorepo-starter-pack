use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(ListItem)]
pub fn list_item(props: &Props) -> Html {
    let theme_context = use_theme_context();

    html! {
        <li class={css!(
            r#"
                margin-bottom: ${spacing_xxs};
            "#,
            spacing_xxs = theme_context.theme.spacings.xxs.clone(),
        )}>
            { props.children.clone() }
        </li>
    }
}
