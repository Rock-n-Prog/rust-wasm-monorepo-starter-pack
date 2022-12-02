use yew::prelude::*;

use crate::styles::theme::types::theme_kind::ThemeKind;

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeContext {
    theme_kind: UseStateHandle<ThemeKind>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

// TODO: FC?
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &Props) -> Html {
    let theme_kind = use_state(|| ThemeKind::Dark);
    // TODO: Provide current theme values
    let theme_ctx = ThemeContext { theme_kind };

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            { props.children.clone() }
        </ContextProvider<ThemeContext>>
    }
}