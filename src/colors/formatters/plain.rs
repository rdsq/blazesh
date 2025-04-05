use crate::colors::esc::esc_sequence as esc;
use crate::colors::formatters::formatter_trait::Formatter;

pub struct PlainFormatter {
    colors: Vec<char>,
}

impl Formatter for PlainFormatter {
    fn format_str(&self, text: &str) -> String {
        let num_items = self.colors.len();
        if num_items == 0 {
            return text.to_string();
        }
        if num_items == 1 {
            // do not waste any stuff
            return format!(
                "{}{}{}",
                esc(&format!("3{}m", self.colors[0])),
                text,
                esc("0m"),
            );
        }
        let mut variation = 0;
        let mut res = String::new();
        let mut prev: &char = &' '; // anything
        for ch in text.chars() {
            let col = &self.colors[variation];
            if col !=   prev {
                // optimize the number of escape sequences for repeating colors
                res.push_str(&format!(
                    "{}{}",
                    esc(&format!("3{}m", col)),
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
        res.push_str(&esc("0m"));
        res
    }
}

// for getting the data
fn verify_and_extract(chunk: &str) -> Option<char> {
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
            return Some(v);
        }
    }
    None
}

impl PlainFormatter {
    pub fn from_conf(conf: &str) -> Self {
        let mut res: Vec<char> = Vec::new();
        let sp = conf.split_whitespace();
        for chunk in sp {
            if let Some(ch) = verify_and_extract(chunk) {
                res.push(ch);
            } else {
                eprintln!("blazesh: invalid color code at BLAZESH_ACCENT_COLOR: {}", chunk);
            }
        }
        if res.is_empty() {
            res.push('6'); // default
        }
        Self { colors: res }
    }
}
