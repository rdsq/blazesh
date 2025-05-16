mod cli;
mod exit_codes;
mod path_display;
mod jobs;
mod git;
mod dir_representation;
mod colors;
mod prompt;

fn main() {
    prompt::prompt();
}
