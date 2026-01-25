#[derive(Debug, Clone, clap::ValueEnum)]
pub enum EscSeqFormat {
    Bash,
    Zsh,
    Fish,
    Csh,
}

impl EscSeqFormat {
    pub fn wrap(&self, seq: &str) -> String {
        match self {
            Self::Bash => format!("\\[{}\\]", seq),
            Self::Zsh | Self::Csh => format!("%{{{}%}}", seq),
            Self::Fish => seq.to_owned(),
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
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Csh => "csh",
        }
    }
}
