pub fn esc_sequence(sequence: &str) -> String {
    format!("%{{\x1b[{}%}}", sequence)
}

pub fn color(code: &str, text: &str) -> String {
    format!(
        "{}{}{}",
        esc_sequence(&format!("{}m", code)),
        text,
        esc_sequence("0m"),
    )
}
