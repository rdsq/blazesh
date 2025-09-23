use super::formatter_trait::Formatter;

pub struct NoFormatter {}

impl <'a>Formatter<'a> for NoFormatter {
    fn format_str(&self, text: &str) -> String {
        text.to_owned()
    }
}

impl NoFormatter {
    pub fn new() -> Self {
        Self {}
    }
}
