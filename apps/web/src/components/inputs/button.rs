use yew::html::onclick::Event;
use yew::prelude::*;
use stylist::yew::styled_component;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;
use crate::styles::theme::types::colors::Colors;

#[derive(Clone, Debug, PartialEq)]
pub enum ButtonVariant {
    Contained,
    Outlined,
    Text,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub variant: Option<ButtonVariant>,
    pub onclick: Option<Callback<Event>>,
}

struct ButtonColors {
    border: String,
    background: String,
    on_background: String,
    hover_background: String,
    hover_on_background: String,
    hover_active_background: String,
    hover_active_on_background: String,
}

// TODO: Add disabled logic (https://github.com/Rock-n-Prog/web-ts-monorepo-starter-pack/blob/main/packages/web-ui/components/inputs/Button.tsx)
fn get_colors(variant: ButtonVariant, colors: Colors) -> ButtonColors {
     match variant {
        ButtonVariant::Contained => ButtonColors {
            border: "transparent".to_string(),
            background: colors.palette.primary.main.clone(),
            on_background: colors.on_primary.clone(),
            hover_background: colors.palette.primary.light.clone(),
            hover_on_background: colors.on_primary.clone(),
            hover_active_background: colors.palette.primary.main.clone(),
            hover_active_on_background: colors.on_primary.clone(),
        },
         ButtonVariant::Outlined => ButtonColors {
             border: colors.palette.primary.main.clone(),
             background: "transparent".to_string(),
             on_background: colors.palette.primary.main.clone(),
             hover_background: colors.palette.primary.main.clone(),
             hover_on_background: colors.on_primary.clone(),
             hover_active_background: colors.palette.primary.light.clone(),
             hover_active_on_background: colors.on_primary.clone(),
         },
         ButtonVariant::Text => ButtonColors {
             border: "transparent".to_string(),
             background: "transparent".to_string(),
             on_background: colors.palette.primary.main.clone(),
             hover_background: "transparent".to_string(),
             hover_on_background: colors.palette.primary.light.clone(),
             hover_active_background: "transparent".to_string(),
             hover_active_on_background: colors.palette.primary.main.clone(),
         }
     }
}

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let theme_context = use_theme_context();
    let colors = use_memo(|colors| get_colors(match props.variant.clone() {
        Some(variant) => variant,
        None => ButtonVariant::Contained,
    }, colors.clone()), theme_context.theme.colors.clone());

    // TODO: Add this
    /*
    &:hover {
        color: ${hover_on_background};
        background-color: ${hover_background};

        &:active {
            color: ${hover_active_on_background};
            background-color: ${hover_active_background};
        }
    }
    */

    // TODO: Add the following
    // cursor: ${disabled ? 'not-allowed' : 'pointer'};
    // font-weight: ${theme.fonts.weights.regular};
    // font-size: ${theme.fonts.sizes.s};
    html! {
        <button class={css!(
            r#"
                display: flex;
                border-radius: ${spacing_xxs};
                padding: ${spacing_xxs} ${spacing_xs};
                outline: 0;
                color: ${on_background};
                background-color: ${background};
                font-family: sans-serif;
                text-transform: uppercase;
                line-height: 1.75;
                border: solid 1px ${border};
                cursor: pointer;
            "#,
            border = colors.border.clone(),
            background = colors.background.clone(),
            on_background = colors.on_background.clone(),
            // TODO: Add hover values
            // hover_background = colors.hover_background.clone(),
            // hover_on_background = colors.hover_on_background.clone(),
            // hover_active_background = colors.hover_active_background.clone(),
            // hover_active_on_background = colors.hover_active_on_background.clone(),
            spacing_xxs = theme_context.theme.spacings.xxs.clone(),
            spacing_xs = theme_context.theme.spacings.xxs.clone(),
        )}
            onclick={props.onclick.clone()}
        >
            { props.text.clone() }
        </button>
    }
}
