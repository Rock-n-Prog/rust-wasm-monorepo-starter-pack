use super::breakpoints::Breakpoints;
use super::colors::Colors;
use super::fonts::Fonts;
use super::spacings::Spacings;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub breakpoints: Breakpoints,
    pub colors: Colors,
    pub fonts: Fonts,
    pub spacings: Spacings,
}