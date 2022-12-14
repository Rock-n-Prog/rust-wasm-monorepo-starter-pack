use crate::styles::theme::types::theme::Theme;
use crate::styles::theme::types::theme_kind::ThemeKind;
use crate::styles::theme::utils::colors::get_colors::get_colors;
use crate::styles::theme::utils::get_fonts::get_fonts;
use super::get_breakpoints::get_breakpoints;
use super::get_spacings::get_spacings;

pub fn get_theme(theme_kind: &ThemeKind) -> Theme {
    Theme {
        breakpoints: get_breakpoints(),
        colors: get_colors(theme_kind),
        fonts: get_fonts(),
        spacings: get_spacings(),
    }
}