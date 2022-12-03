use crate::styles::theme::types::colors::{Colors, Variants, VariantColors, DisabledColors};
use super::get_palette::get_palette;

pub fn get_dark_colors() -> Colors {
    Colors {
        palette: get_palette(),
        background: "#121212".to_string(),
        surface: "#222222".to_string(),
        on_primary: "#fff".to_string(),
        on_secondary: "#fff".to_string(),
        on_variant: "#fff".to_string(),
        on_background: "#fff".to_string(),
        on_surface: "#fff".to_string(),
        variants: Variants {
            success: VariantColors {
                main: "#66bb6a".to_string(),
                background: "#0c130d".to_string(),
                on_background: "#cce8cd".to_string(),
            },
            info: VariantColors {
                main: "#29b6f6".to_string(),
                background: "#071318".to_string(),
                on_background: "#b8e7fb".to_string(),
            },
            warning: VariantColors {
                main: "#ffa726".to_string(),
                background: "#191207".to_string(),
                on_background: "#ffe2b7".to_string(),
            },
            error: VariantColors {
                main: "#f44336".to_string(),
                background: "#160b0b".to_string(),
                on_background: "#f4c7c7".to_string(),
            },
        },
        disabled: DisabledColors {
            on_background: "rgba(255, 255, 255, 0.3)".to_string(),
            background: "rgba(255, 255, 255, 0.12)".to_string(),
        }
    }
}