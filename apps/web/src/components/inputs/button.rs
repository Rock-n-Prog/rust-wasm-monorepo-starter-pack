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
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(ButtonVariant::Contained)]
    pub variant: ButtonVariant,
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

// TODO: Improve this Option for hover/active in ButtonColors
fn get_colors(variant: ButtonVariant, disabled: bool, colors: Colors) -> ButtonColors {
     match (variant, disabled) {
         (ButtonVariant::Contained, false) => ButtonColors {
            border: "transparent".to_string(),
            background: colors.palette.primary.main.clone(),
            on_background: colors.on_primary.clone(),
            hover_background: colors.palette.primary.light.clone(),
            hover_on_background: colors.on_primary.clone(),
            hover_active_background: colors.palette.primary.main.clone(),
            hover_active_on_background: colors.on_primary,
        },
         (ButtonVariant::Outlined, false) => ButtonColors {
             border: colors.palette.primary.main.clone(),
             background: "transparent".to_string(),
             on_background: colors.palette.primary.main.clone(),
             hover_background: colors.palette.primary.main.clone(),
             hover_on_background: colors.on_primary.clone(),
             hover_active_background: colors.palette.primary.light.clone(),
             hover_active_on_background: colors.on_primary,
         },
         (ButtonVariant::Text, false) => ButtonColors {
             border: "transparent".to_string(),
             background: "transparent".to_string(),
             on_background: colors.palette.primary.main.clone(),
             hover_background: "transparent".to_string(),
             hover_on_background: colors.palette.primary.light.clone(),
             hover_active_background: "transparent".to_string(),
             hover_active_on_background: colors.palette.primary.main,
         },
         (ButtonVariant::Contained, true) => ButtonColors {
             border: "transparent".to_string(),
             background: colors.disabled.background.clone(),
             on_background:  colors.disabled.on_background.clone(),
             hover_background:  colors.disabled.background.clone(),
             hover_on_background:  colors.disabled.on_background.clone(),
             hover_active_background:  colors.disabled.background.clone(),
             hover_active_on_background:  colors.disabled.on_background,
         },
         (ButtonVariant::Outlined, true) => ButtonColors {
             border:  colors.disabled.on_background.clone(),
             background: "transparent".to_string(),
             on_background:  colors.disabled.on_background.clone(),
             hover_background:  "transparent".to_string(),
             hover_on_background:  colors.disabled.on_background.clone(),
             hover_active_background:  "transparent".to_string(),
             hover_active_on_background:  colors.disabled.on_background,
         },
         (ButtonVariant::Text, true) => ButtonColors {
             border: "transparent".to_string(),
             background: "transparent".to_string(),
             on_background: colors.disabled.on_background.clone(),
             hover_background: "transparent".to_string(),
             hover_on_background: colors.disabled.on_background.clone(),
             hover_active_background: "transparent".to_string(),
             hover_active_on_background: colors.disabled.on_background,
         },
     }
}

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let theme_context = use_theme_context();
    let colors = use_memo(|colors| get_colors(props.variant.clone(), props.disabled, colors.clone()), theme_context.theme.colors.clone());

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
                cursor: ${cursor};
                font-weight: ${font_weight_regular};
                font-size: ${font_size_s};

                &:hover {
                    color: ${hover_on_background};
                    background-color: ${hover_background};
                }

                &:hover:active {
                    color: ${hover_active_on_background};
                    background-color: ${hover_active_background};
                }
            "#,
            font_weight_regular = theme_context.theme.fonts.weights.regular.clone(),
            font_size_s = theme_context.theme.fonts.sizes.s.clone(),
            cursor = if props.disabled { "not-allowed".to_string() } else { "pointer".to_string() },
            border = colors.border.clone(),
            background = colors.background.clone(),
            on_background = colors.on_background.clone(),
            hover_background = colors.hover_background.clone(),
            hover_on_background = colors.hover_on_background.clone(),
            hover_active_background = colors.hover_active_background.clone(),
            hover_active_on_background = colors.hover_active_on_background.clone(),
            spacing_xxs = theme_context.theme.spacings.xxs.clone(),
            spacing_xs = theme_context.theme.spacings.xxs.clone(),
        )}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        >
            { props.text.clone() }
        </button>
    }
}
