use super::breakpoints::Breakpoints;
use super::colors::Colors;
use super::spacings::Spacings;

#[derive(Debug, Clone)]
pub struct Theme {
    pub breakpoints: Breakpoints,
    pub colors: Colors,
    pub spacings: Spacings,
}