use std::env;
use crate::colors::formatters::{formatter_trait, plain, gradient};

pub fn get_formatter() -> Box<dyn formatter_trait::Formatter> {
    if let Some(custom_unchecked) = env::var_os("BLAZESH_ACCENT_COLOR") {
        if let Some(as_str) = custom_unchecked.to_str() {
            // for now no distinguishing logic
            if as_str.starts_with("gradient ") {
                if let Some(gradient_formatter) = gradient::GradientFormatter::from_conf(as_str) {
                    return Box::new(gradient_formatter);
                    // if it fails go to default
                }
            }
            return Box::new(plain::PlainFormatter::from_conf(as_str));
        }
    }
    // default
    Box::new(plain::PlainFormatter::from_conf(""))
}
