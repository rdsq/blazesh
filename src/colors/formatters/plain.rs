use super::formatter_trait::Formatter;
use crate::colors::terminal_color::TerminalColor;
use crate::colors::escseq::EscSeqFormat;

pub struct PlainFormatter<'a> {
    colors: Vec<TerminalColor>,
    escformat: &'a EscSeqFormat,
}

impl <'a>Formatter<'a> for PlainFormatter<'a> {
    fn format_str(&self, text: &str) -> String {
        let num_items = self.colors.len();
        if num_items == 0 {
            return text.to_string();
        }
        if num_items == 1 {
            // do not waste any stuff
            return format!(
                "{}{}{}",
                self.colors[0].to_ansi_foreground(&self.escformat),
                text,
                self.escformat.esc("0m"),
            );
        }
        let mut variation = 0;
        let mut res = String::new();
        let mut prev: &TerminalColor = &TerminalColor::Ansi(' '); // anything
        for ch in text.chars() {
            let col = &self.colors[variation];
            if col != prev {
                // optimize the number of escape sequences for repeating colors
                res.push_str(&format!(
                    "{}{}",
                    col.to_ansi_foreground(&self.escformat),
                    ch,
                ));
            } else {
                res.push(ch); // just the char
            }
            prev = col;
            variation = variation + 1;
            if variation > (num_items - 1) {
                variation = 0; // reset
            }
        }
        res.push_str(&self.escformat.esc("0m"));
        res
    }
}

impl <'a>PlainFormatter<'a> {
    pub fn from_conf(conf: &str, escformat: &'a EscSeqFormat) -> Option<Self> {
        let mut res: Vec<TerminalColor> = Vec::new();
        let sp = conf.split_whitespace();
        for chunk in sp {
            if let Some(col) = TerminalColor::try_parse(chunk) {
                res.push(col);
            } else {
                eprintln!("blazesh: invalid color code at BLAZESH_ACCENT_COLOR: {}", chunk);
            }
        }
        if res.is_empty() {
            return None;
        }
        Some(Self { colors: res, escformat })
    }
}
