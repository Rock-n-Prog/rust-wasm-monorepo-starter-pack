#[derive(Debug, Clone, PartialEq)]
pub struct PaletteVariant {
    pub light: String,
    pub main: String,
    pub dark: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    pub primary: PaletteVariant,
    pub secondary: PaletteVariant,
}