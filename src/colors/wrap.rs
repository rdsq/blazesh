use once_cell::sync::Lazy;

fn wrap_bash(seq: &str) -> String {
    format!("\\[{}\\]", seq)
}

fn wrap_zsh(seq: &str) -> String {
    format!("%{{{}%}}", seq)
}

fn wrap_unknown(seq: &str) -> String {
    seq.to_string()
}

pub static WRAP_SEQ: Lazy<fn(&str) -> String> = Lazy::new(|| {
    let sh = std::env::var("_BLAZESH_SHELL").ok();
    return if sh == Some("bash".to_string()) {
        wrap_bash
    } else if sh == Some("zsh".to_string()) {
        wrap_zsh
    } else {
        eprintln!("blazesh: unknown shell");
        wrap_unknown
    }
});
