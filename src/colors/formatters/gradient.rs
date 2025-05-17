use crate::colors::esc::esc_sequence;
use crate::colors::wrap::WRAP_SEQ;
use crate::colors::formatters::formatter_trait::Formatter;
use crate::colors::rgb::RGB;
use crate::colors::gradient::gradient;

pub struct GradientFormatter {
    pub colors: Vec<RGB>,
    pub interval: Option<f32>,
}

impl Formatter for GradientFormatter {
    fn format_str(&self, text: &str) -> String {
        let mut result = String::new();
        let mut prev = None;
        for (i, ch) in text.chars().enumerate() {
            let interval = self.interval.unwrap_or(text.len() as f32);
            let t = (i as f32 / (interval - 1.0)) * (self.colors.len() - 1) as f32;
            let color = gradient(&self.colors, t);
            let color_id_i_guess = color.to_ansi256();
            if prev != Some(color_id_i_guess) {
                // optimization
                result.push_str(
                    &WRAP_SEQ(&color.to_ansi_foreground())
                );
                prev = Some(color_id_i_guess);
            }
            result.push(ch);
        }
        result.push_str(&esc_sequence("0m"));
        result
    }
}

impl GradientFormatter {
    pub fn from_conf(conf: &str) -> Option<Self> {
        let mut sp = conf.split_whitespace();
        sp.next(); // skip `gradient` keyword
        let mut interval = None;
        let mut colors = Vec::new();
        for i in sp {
            if let Some((kw, interval_inp)) = i.split_once('=') {
                if kw == "interval" {
                    if let Ok(interval_val) = interval_inp.parse::<f32>() {
                        interval = Some(interval_val);
                        continue;
                    }
                }
            }
            if let Some(color) = RGB::try_parse(i) {
                colors.push(color);
            }
        }
        if colors.len() < 2 {
            return None;
        }
        Some(Self { colors, interval })
    }
}
