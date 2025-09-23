#[derive(Debug, Clone, clap::ValueEnum)]
pub enum EscSeqFormat {
    Other,
    Bash,
    Zsh,
}

impl EscSeqFormat {
    pub fn wrap(&self, seq: &str) -> String {
        match self {
            Self::Other => seq.to_owned(),
            Self::Bash => format!("\\[{}\\]", seq),
            Self::Zsh => format!("%{{{}%}}", seq),
        }
    }
    pub fn esc(&self, seq: &str) -> String {
        self.wrap(&format!("\x1b[{}", seq))
    }
    pub fn color(&self, code: &str, text: &str) -> String {
        format!(
            "{}{}{}",
            self.esc(&format!("{}m", code)),
            text,
            self.esc("0m"),
        )
    }
}
