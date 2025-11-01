use crate::colors::{
    escseq::EscSeqFormat,
    formatters::formatter_trait::Formatter,
    gradient::gradient,
    misc::{to_ansi_foreground, try_parse_hex},
};
use rgb::RGB8;

pub struct GradientFormatter<'a> {
    pub colors: Vec<RGB8>,
    pub interval: Option<f32>,
    pub escformat: &'a EscSeqFormat,
}

impl <'a>Formatter<'a> for GradientFormatter<'a> {
    fn format_str(&self, text: &str) -> String {
        if text.len() == 1 {
            return format!(
                "{}{}{}",
                self.escformat.wrap(&to_ansi_foreground(&self.colors[0])),
                text,
                self.escformat.esc("0m"),
            );
        }
        let mut result = String::new();
        let mut prev = None;
        for (i, ch) in text.chars().enumerate() {
            let interval = self.interval.unwrap_or(text.len() as f32);
            let t = (i as f32 / (interval - 1.0)) * (self.colors.len() - 1) as f32;
            let color = gradient(&self.colors, t);
            let color_id_i_guess = color.clone();
            if prev.as_ref() != Some(&color_id_i_guess) {
                // optimization
                result.push_str(
                    &self.escformat.wrap(&to_ansi_foreground(&color))
                );
                prev = Some(color_id_i_guess);
            }
            result.push(ch);
        }
        result.push_str(&self.escformat.esc("0m"));
        result
    }
}

impl <'a>GradientFormatter<'a> {
    pub fn from_conf(conf: &str, escformat: &'a EscSeqFormat) -> Option<Self> {
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
            if let Some(color) = try_parse_hex(i) {
                colors.push(color);
            }
        }
        if colors.len() < 2 {
            return None;
        }
        Some(Self { colors, interval, escformat })
    }
}
