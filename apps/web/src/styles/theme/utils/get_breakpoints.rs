use crate::styles::theme::types::breakpoints::Breakpoints;

// Min-widths
pub fn get_breakpoints() -> Breakpoints {
    return Breakpoints {
        xs: "575px".to_string(),
        s: "767px".to_string(),
        m: "991px".to_string(),
        l: "1199px".to_string(),
        xl: "1399px".to_string(),
    }
}