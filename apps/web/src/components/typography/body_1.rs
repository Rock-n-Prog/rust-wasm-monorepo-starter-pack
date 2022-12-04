use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or(false)]
    pub no_margin: bool,
}

#[styled_component(Body1)]
pub fn body_1(props: &Props) -> Html {
    let theme_context = use_theme_context();

    html! {
        <p class={css!(
            r#"
                margin: ${margin};
            "#,
            margin = if props.no_margin { "0".to_string() } else { format!("0 0 {margin_bottom} 0", margin_bottom = theme_context.theme.spacings.xxs.clone()) },
        )}>
            { props.children.clone() }
        </p>
    }
}
