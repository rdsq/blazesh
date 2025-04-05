pub trait Formatter {
    fn format_str(&self, text: &str) -> String;
}
