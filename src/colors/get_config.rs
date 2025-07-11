use std::env;
use super::{formatters::{formatter_trait, gradient, plain, no_format}, rgb::RGB};

pub fn get_formatter() -> Box<dyn formatter_trait::Formatter> {
    if let Ok(custom) = env::var("BLAZESH_ACCENT_COLOR") {
        if custom.to_lowercase() == "none" {
            return Box::new(no_format::NoFormatter::new());
        }
        if custom.starts_with("gradient ") {
            if let Some(gradient_formatter) = gradient::GradientFormatter::from_conf(&custom) {
                return Box::new(gradient_formatter);
                // if it fails go to default
            }
        }
        if let Some(plain_formatter) = plain::PlainFormatter::from_conf(&custom) {
            return Box::new(plain_formatter);
        }
    }
    // default
    Box::new(gradient::GradientFormatter {
        colors: vec![
            RGB { r: 255, g: 153, b: 0 }, // FF9900
            RGB { r: 255, g: 255, b: 0 }, // FFFF00
        ],
        interval: Some(10.0),
    })
}
