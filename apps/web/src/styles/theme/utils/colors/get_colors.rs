use crate::styles::theme::types::colors::Colors;
use crate::styles::theme::types::theme_kind::ThemeKind;
use super::get_light_colors::get_light_colors;
use super::get_dark_colors::get_dark_colors;

pub fn get_colors(theme_kind: &ThemeKind) -> Colors {
    match theme_kind {
        ThemeKind::Light => get_light_colors(),
        ThemeKind::Dark => get_dark_colors(),
    }
}