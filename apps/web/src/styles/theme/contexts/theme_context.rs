use std::rc::Rc;
use yew::prelude::*;
use crate::styles::theme::types::theme::Theme;
use crate::styles::theme::types::theme_kind::ThemeKind;
use crate::styles::theme::utils::get_theme::get_theme;

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Rc<Theme>,
    pub theme_kind: UseStateHandle<ThemeKind>,
    pub switch_theme: Callback<()>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &Props) -> Html {
    let theme_kind = use_state(|| ThemeKind::Dark);
    let theme = use_memo(|kind| get_theme(kind), theme_kind.clone());
    let switch_theme = {
        let theme_kind = theme_kind.clone();
        Callback::from(move |_| theme_kind.set(if *theme_kind == ThemeKind::Light { ThemeKind::Dark } else { ThemeKind::Light }))
    };

    let theme_context = ThemeContext { theme, theme_kind, switch_theme };

    html! {
        <ContextProvider<ThemeContext> context={theme_context}>
            { props.children.clone() }
        </ContextProvider<ThemeContext>>
    }
}