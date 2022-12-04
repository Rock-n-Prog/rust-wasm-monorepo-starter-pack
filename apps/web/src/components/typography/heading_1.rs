use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(Heading1)]
pub fn heading_1(props: &Props) -> Html {
    let theme_context = use_theme_context();

    // TODO: Add typography styles (https://github.com/Rock-n-Prog/web-ts-monorepo-starter-pack/blob/main/packages/web-ui/components/typography/Typography.tsx)
    html! {
        <h1 class={css!(
            r#"
                font-size: ${font_size_xxl};
                margin-bottom: ${spacing_s};
            "#,
            font_size_xxl = theme_context.theme.fonts.sizes.xxl.clone(),
            spacing_s = theme_context.theme.spacings.s.clone(),
        )}>
            { props.children.clone() }
        </h1>
    }
}
