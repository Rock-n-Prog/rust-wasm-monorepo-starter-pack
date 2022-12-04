#[derive(Debug, Clone, PartialEq)]
pub struct FontFamilies {
    pub sans_serif: String,
    pub serif: String,
    pub monospaced: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontSizes {
    pub xxs: String,
    pub xs: String,
    pub s: String,
    pub m: String,
    pub l: String,
    pub xl: String,
    pub xxl: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontWeights {
    pub light: String,
    pub regular: String,
    pub medium: String,
    pub semi_bold: String,
    pub bold: String,
    pub bolder: String,
}

// https://material.io/design/color/text-legibility.html#text-backgrounds
#[derive(Debug, Clone, PartialEq)]
pub struct FontEmphasis {
    pub high: String,
    pub medium: String,
    pub disabled: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fonts {
    pub families: FontFamilies,
    pub sizes: FontSizes,
    pub weights: FontWeights,
    pub emphasis: FontEmphasis,
}