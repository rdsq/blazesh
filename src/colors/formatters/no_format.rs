use super::formatter_trait::Formatter;

pub struct NoFormatter {}

impl Formatter for NoFormatter {
    fn format_str(&self, text: &str) -> String {
        text.to_owned()
    }
}

impl NoFormatter {
    pub fn new() -> Self {
        Self {}
    }
}
