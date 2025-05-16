#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum Shell {
    Bash,
    Zsh,
    Detect,
    Fish,
}

#[derive(clap::Parser, Debug)]
/// Generate shell code to add Blazesh prompt
pub struct Setup {
    /// The shell used
    shell: Shell,
}

const CONSTRUCT_PROMPT: &str = include_str!("./snippets/construct-prompt.sh");
const BASH: &str = include_str!("./snippets/bash.sh");
const ZSH: &str = include_str!("./snippets/zsh.sh");

pub fn setup(args: Setup) {
    match args.shell {
        Shell::Bash => {
            print!("{}", CONSTRUCT_PROMPT);
            print!("{}", BASH);
        },
        Shell::Zsh => {
            print!("{}", CONSTRUCT_PROMPT);
            print!("{}", ZSH);
        },
        Shell::Detect => print!(
            include_str!("./snippets/detect.sh"),
            CONSTRUCT_PROMPT,
            BASH,
            ZSH,
        ),
        Shell::Fish => print!("{}", include_str!("./snippets/fish.fish")),
    };
}
