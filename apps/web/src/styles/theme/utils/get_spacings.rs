use crate::styles::theme::types::spacings::Spacings;

pub fn get_spacings() -> Spacings {
    Spacings {
        xxs: ".25rem".to_string(), // 4 px
        xs: ".5rem".to_string(), // 8 px
        s: ".75rem".to_string(), // 12 px
        m: "1rem".to_string(), // 16 px
        l: "1.5rem".to_string(), // 24 px
        xl: "2.25rem".to_string(), // 36 px
    }
}