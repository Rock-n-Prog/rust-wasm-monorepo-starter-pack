use crate::styles::theme::types::palette::{Palette, PaletteVariant};
use crate::styles::theme::types::colors::{Colors, Variants, VariantColors, DisabledColors};
use crate::styles::theme::types::theme_kind::ThemeKind;

// https://coolors.co/c046d3-b118c8-7b108c-374766-061a40-04122c
fn get_palette() -> Palette {
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

fn get_light_colors() -> Colors {
    return Colors {
        palette: get_palette(),
        background: "#fff".to_string(),
        surface: "#fafafa".to_string(),
        on_primary: "#fff".to_string(),
        on_secondary: "#fff".to_string(),
        on_variant: "#fff".to_string(),
        on_background: "#000".to_string(),
        on_surface: "#000".to_string(),
        variants: Variants {
            success: VariantColors {
                main: "#4CAF50".to_string(),
                background: "#edf7ed".to_string(),
                on_background: "#1e4620".to_string(),
            },
            info: VariantColors {
                main: "#03A9F4".to_string(),
                background: "#e5f6fd".to_string(),
                on_background: "#014361".to_string(),
            },
            warning: VariantColors {
                main: "#FF9800".to_string(),
                background: "#fff4e5".to_string(),
                on_background: "#663C00".to_string(),
            },
            error: VariantColors {
                main: "#EF5350".to_string(),
                background: "#fdeded".to_string(),
                on_background: "#5F2120".to_string(),
            },
        },
        disabled: DisabledColors {
            on_background: "rgba(0, 0, 0, 0.26)".to_string(),
            background: "rgba(0, 0, 0, 0.12)".to_string(),
        }
    }
}

pub fn get_colors(theme_kind: &ThemeKind) -> Colors {
    match theme_kind {
        ThemeKind::Light => get_light_colors(),
        // TODO: Dark colors (https://github.com/Rock-n-Prog/web-ts-monorepo-starter-pack/blob/main/packages/theme/index.ts)
        ThemeKind::Dark => get_light_colors(),
    }
}