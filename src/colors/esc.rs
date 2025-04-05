pub fn wrap_seq(sequence: &str) -> String {
    format!("%{{{}%}}", sequence)
}

pub fn esc_sequence(sequence: &str) -> String {
    wrap_seq(&format!("\x1b[{}", sequence))
}

pub fn color(code: &str, text: &str) -> String {
    format!(
        "{}{}{}",
        esc_sequence(&format!("{}m", code)),
        text,
        esc_sequence("0m"),
    )
}
