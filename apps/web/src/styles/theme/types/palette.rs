#[derive(Debug, Clone)]
pub struct PaletteVariant {
    pub light: String,
    pub main: String,
    pub dark: String,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub primary: PaletteVariant,
    pub secondary: PaletteVariant,
}