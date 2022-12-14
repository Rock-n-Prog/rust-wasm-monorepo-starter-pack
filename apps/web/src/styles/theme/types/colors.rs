use super::palette::Palette;

#[derive(Debug, Clone, PartialEq)]
pub struct VariantColors {
    pub main: String,
    pub background: String,
    pub on_background: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisabledColors {
    pub on_background: String,
    pub background: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variants {
    pub success: VariantColors,
    pub info: VariantColors,
    pub warning: VariantColors,
    pub error: VariantColors,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Colors {
    pub palette: Palette,
    pub background: String,
    pub surface: String,
    pub on_primary: String,
    pub on_secondary: String,
    pub on_variant: String,
    pub on_background: String,
    pub on_surface: String,
    pub variants: Variants,
    pub disabled: DisabledColors,
}