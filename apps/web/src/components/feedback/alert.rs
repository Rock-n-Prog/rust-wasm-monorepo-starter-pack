use yew::prelude::*;
use stylist::yew::styled_component;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;
use crate::styles::theme::types::colors::Variants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub text: String,
    pub severity: Severity,
}

struct Colors {
    background: String,
    on_background: String,
}

fn get_colors(severity: Severity, variants: Variants) -> Colors {
     match severity {
        Severity::Info => Colors {
            background: variants.info.background,
            on_background: variants.info.on_background,
        },
        Severity::Success => Colors {
            background: variants.success.background,
            on_background: variants.success.on_background,
        },
        Severity::Warning => Colors {
            background: variants.warning.background,
            on_background: variants.warning.on_background,
        },
        Severity::Error => Colors {
            background: variants.error.background,
            on_background: variants.error.on_background,
        },
    }
}

#[styled_component(Alert)]
pub fn alert(props: &Props) -> Html {
    let theme_context = use_theme_context();
    let colors = use_memo(|variants| get_colors(props.severity.clone(), variants.clone()), theme_context.theme.colors.variants.clone());

    html! {
        <div class={css!(
            r#"
                background-color: ${background};
                color: ${on_background};
                padding: ${spacing_s} ${spacing_m};
                border-radius: ${spacing_xxs};
            "#,
            background = colors.background.clone(),
            on_background = colors.on_background.clone(),
            spacing_xxs = theme_context.theme.spacings.xxs.clone(),
            spacing_s = theme_context.theme.spacings.s.clone(),
            spacing_m = theme_context.theme.spacings.m.clone(),
        )}>
            <p>{ props.text.clone() }</p>
        </div>
    }
}
