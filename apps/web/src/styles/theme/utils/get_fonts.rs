use crate::styles::theme::types::fonts::{Fonts, FontFamilies, FontSizes, FontWeights, FontEmphasis};

fn get_families() -> FontFamilies {
    FontFamilies {
        sans_serif: r#""Helvetica Neue", Helvetica, Arial, sans-serif'"#.to_string(),
        serif: r#"Georgia, Times, "Times New Roman", serif"#.to_string(),
        monospaced: "Consolas, monaco, monospace".to_string()
    }
}

fn get_sizes() -> FontSizes {
    FontSizes {
        xxs: ".625rem".to_string(), // 10 px
        xs: ".75rem".to_string(), // 12 px
        s: ".875rem".to_string(), // 14 px
        m: "1rem".to_string(), // 16 px
        l: "1.5rem".to_string(), // 24 px
        xl: "2.25rem".to_string(), // 36 px
        xxl: "3rem".to_string(), // 48 px
    }
}

fn get_weights() -> FontWeights {
    FontWeights {
        light: "300".to_string(),
        regular: "400".to_string(),
        medium: "500".to_string(),
        semi_bold: "600".to_string(),
        bold: "700".to_string(),
        bolder: "800".to_string(),
    }
}

fn get_emphasis() -> FontEmphasis {
    FontEmphasis {
        high: "0.87".to_string(),
        medium: "0.6".to_string(),
        disabled: "0.38".to_string(),
    }
}

pub fn get_fonts() -> Fonts {
    Fonts {
        families: get_families(),
        sizes: get_sizes(),
        weights: get_weights(),
        emphasis: get_emphasis(),
    }
}