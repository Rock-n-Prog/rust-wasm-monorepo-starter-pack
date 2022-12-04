use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(Heading2)]
pub fn heading_2(props: &Props) -> Html {
    let theme_context = use_theme_context();

    // TODO: Add typography styles (https://github.com/Rock-n-Prog/web-ts-monorepo-starter-pack/blob/main/packages/web-ui/components/typography/Typography.tsx)
    html! {
        <h2 class={css!(
            r#"
                font-size: ${font_size_xl};
                margin-bottom: ${spacing_xs};
            "#,
            font_size_xl = theme_context.theme.fonts.sizes.xl.clone(),
            spacing_xs = theme_context.theme.spacings.xs.clone(),
        )}>
            { props.children.clone() }
        </h2>
    }
}
