use super::rgb::RGB;
use super::escseq::EscSeqFormat;

#[derive(PartialEq)]
pub enum TerminalColor {
    Ansi(char),
    Rgb(RGB),
}

impl TerminalColor {
    pub fn to_ansi_foreground(&self, escformat: &EscSeqFormat) -> String {
        return match self {
            Self::Ansi(c) => escformat.esc(&format!("3{}m", c)),
            Self::Rgb(rgb) => escformat.wrap(&rgb.to_ansi_foreground()),
        };
    }
    pub fn try_parse(chunk: &str) -> Option<Self> {
        if let Some(parsed_rgb) = RGB::try_parse(&chunk) {
            return Some(Self::Rgb(parsed_rgb));
        }
        if chunk.len() == 1 {
            let v = chunk
                .chars().nth(0).unwrap();
            // verifying
            if v == '0'
            || v == '1'
            || v == '2'
            || v == '3'
            || v == '4'
            || v == '5'
            || v == '6'
            || v == '7'
            || v == '9' {
                return Some(Self::Ansi(v));
            }
        }
        None
    }
}
