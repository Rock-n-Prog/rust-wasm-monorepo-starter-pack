use crate::styles::theme::types::colors::{Colors, Variants, VariantColors, DisabledColors};

use super::get_palette::get_palette;

pub fn get_light_colors() -> Colors {
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