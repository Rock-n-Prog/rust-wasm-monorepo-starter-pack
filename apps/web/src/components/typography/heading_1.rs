use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or(false)]
    pub no_margin: bool,
    #[prop_or(false)]
    pub small: bool,
}

#[styled_component(Heading1)]
pub fn heading_1(props: &Props) -> Html {
    let theme_context = use_theme_context();

    html! {
        <h1 class={css!(
            r#"
                font-size: ${font_size_xxl};
                margin: ${margin};
            "#,
            font_size_xxl = if props.small { theme_context.theme.fonts.sizes.xl.clone() } else { theme_context.theme.fonts.sizes.xxl.clone() },
            margin = if props.no_margin { "0".to_string() } else { format!("0 0 {margin_bottom} 0", margin_bottom = theme_context.theme.spacings.s.clone()) },
        )}>
            { props.children.clone() }
        </h1>
    }
}
