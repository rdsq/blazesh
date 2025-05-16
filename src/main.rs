mod cli;
mod exit_codes;
mod path_display;
mod jobs;
mod git;
mod dir_representation;
mod colors;
mod prompt;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "blazesh")]
#[command(about = "Custom shell prompt")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Prompt(prompt::Prompt),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prompt(args) => prompt::prompt(args),
    };
}
