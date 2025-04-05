use crate::colors::esc::{esc_sequence, wrap_seq};
use crate::colors::formatters::formatter_trait::Formatter;
use crate::colors::rgb::RGB;

pub struct GradientFormatter {
    start: RGB,
    end: RGB,
}

impl Formatter for GradientFormatter {
    fn format_str(&self, text: &str) -> String {
        let num_items = text.len();
        let mut result = String::new();
        for (i, ch) in text.chars().enumerate() {
            let t = i as f32 / num_items as f32;
            let color = self.start.lerp(&self.end, t);
            result.push_str(
                &format!("{}{}", wrap_seq(&color.to_ansi_foreground()), ch)
            )
        }
        result.push_str(&esc_sequence("0m"));
        result
    }
}

impl GradientFormatter {
    pub fn from_conf(conf: &str) -> Option<Self> {
        let mut sp = conf.split_whitespace();
        sp.next(); // skip `gradient` keyword
        if let (Some(start_str), Some(end_str)) = (sp.next(), sp.next()) {
            if let (Some(start), Some(end)) = (RGB::try_parse(start_str), RGB::try_parse(end_str)) {
                return Some(Self { start, end });
            }
        }
        None
    }
}
