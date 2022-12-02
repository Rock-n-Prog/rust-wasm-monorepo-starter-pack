use crate::styles::theme::types::palette::{Palette, PaletteVariant};

// https://coolors.co/c046d3-b118c8-7b108c-374766-061a40-04122c
pub fn get_palette() -> Palette {
    return Palette {
        primary: PaletteVariant {
            light: "#c046d3".to_string(),
            main: "#B118C8".to_string(),
            dark: "#7b108c".to_string()
        },
        secondary: PaletteVariant {
            light: "#374766".to_string(),
            main: "#061a40".to_string(),
            dark: "#04122c".to_string()
        }
    }
}