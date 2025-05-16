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

pub fn setup(args: Setup) {
    match args.shell {
        Shell::Bash => todo!(),
        Shell::Zsh => todo!(),
        Shell::Detect => print!(include_str!("./snippets/detect.sh")),
        Shell::Fish => todo!(),
    };
}
