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
    pub disabled: Option<bool>,
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

// TODO: Improve this with match (variant, disabled), and/or Option for hover/active in ButtonColors
fn get_colors(variant: ButtonVariant, disabled: bool, colors: Colors) -> ButtonColors {
     match variant {
        ButtonVariant::Contained => ButtonColors {
            border: "transparent".to_string(),
            background: if disabled { colors.disabled.background.clone() } else {colors.palette.primary.main.clone() },
            on_background: if disabled { colors.disabled.on_background.clone() } else {colors.on_primary.clone() },
            hover_background: if disabled { colors.disabled.background.clone() } else {colors.palette.primary.light.clone() },
            hover_on_background: if disabled { colors.disabled.on_background.clone() } else {colors.on_primary.clone() },
            hover_active_background: if disabled { colors.disabled.background.clone() } else {colors.palette.primary.main.clone() },
            hover_active_on_background: if disabled { colors.disabled.on_background } else {colors.on_primary },
        },
         ButtonVariant::Outlined => ButtonColors {
             border: if disabled { colors.disabled.on_background.clone() } else { colors.palette.primary.main.clone() },
             background: "transparent".to_string(),
             on_background: if disabled { colors.disabled.on_background.clone() } else { colors.palette.primary.main.clone() },
             hover_background: if disabled { "transparent".to_string() } else {colors.palette.primary.main.clone() },
             hover_on_background: if disabled { colors.disabled.on_background.clone() } else { colors.on_primary.clone() },
             hover_active_background: if disabled { "transparent".to_string() } else {colors.palette.primary.light.clone() },
             hover_active_on_background: if disabled { colors.disabled.on_background } else { colors.on_primary },
         },
         ButtonVariant::Text => ButtonColors {
             border: "transparent".to_string(),
             background: "transparent".to_string(),
             on_background: if disabled { colors.disabled.on_background.clone() } else { colors.palette.primary.main.clone() },
             hover_background: "transparent".to_string(),
             hover_on_background: if disabled { colors.disabled.on_background.clone() } else { colors.palette.primary.light.clone() },
             hover_active_background: "transparent".to_string(),
             hover_active_on_background: if disabled { colors.disabled.on_background } else { colors.palette.primary.main },
         }
     }
}

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let theme_context = use_theme_context();
    // TODO: Can we use "derivative" or similar to handle default values?
    let disabled = props.disabled.unwrap_or(false);
    let variant = match props.variant.clone() {
        Some(variant) => variant,
        None => ButtonVariant::Contained,
    };
    let colors = use_memo(|colors| get_colors(variant, disabled, colors.clone()), theme_context.theme.colors.clone());

    // TODO: Add the following
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
                cursor: ${cursor};

                &:hover {
                    color: ${hover_on_background};
                    background-color: ${hover_background};
                }

                &:hover:active {
                    color: ${hover_active_on_background};
                    background-color: ${hover_active_background};
                }
            "#,
            cursor = if disabled { "not-allowed".to_string() } else { "pointer".to_string() },
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
            {disabled}
        >
            { props.text.clone() }
        </button>
    }
}
