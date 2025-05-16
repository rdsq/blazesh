#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Unknown,
}
