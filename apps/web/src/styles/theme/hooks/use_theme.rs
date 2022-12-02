use crate::styles::theme::contexts::theme_context::ThemeContext;

#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}