use crate::styles::theme::types::theme::Theme;
use crate::styles::theme::types::theme_kind::ThemeKind;

use super::get_breakpoints::get_breakpoints;
use super::get_colors::get_colors;
use super::get_spacings::get_spacings;

pub fn get_theme(theme_kind: &ThemeKind) -> Theme {
    return Theme {
        breakpoints: get_breakpoints(),
        colors: get_colors(theme_kind),
        spacings: get_spacings(),
    }
}