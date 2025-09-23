pub trait Formatter<'a> {
    fn format_str(&self, text: &str) -> String;
}
