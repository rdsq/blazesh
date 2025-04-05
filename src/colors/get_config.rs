use std::env;
use crate::colors::formatters::{formatter_trait, plain};

pub fn get_formatter() -> Box<dyn formatter_trait::Formatter> {
    if let Some(custom_unchecked) = env::var_os("BLAZESH_ACCENT_COLOR") {
        if let Some(as_str) = custom_unchecked.to_str() {
            // for now no distinguishing logic
            return Box::new(plain::PlainFormatter::new(as_str));
        }
    }
    // default
    Box::new(plain::PlainFormatter::new(""))
}
