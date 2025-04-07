use std::env;
use super::formatters::{formatter_trait, plain, gradient};

pub fn get_formatter() -> Box<dyn formatter_trait::Formatter> {
    if let Ok(custom) = env::var("BLAZESH_ACCENT_COLOR") {
        if custom.starts_with("gradient ") {
            if let Some(gradient_formatter) = gradient::GradientFormatter::from_conf(&custom) {
                return Box::new(gradient_formatter);
                // if it fails go to default
            }
        }
        return Box::new(plain::PlainFormatter::from_conf(&custom));
    }
    // default
    Box::new(plain::PlainFormatter::from_conf(""))
}
