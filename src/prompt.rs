use crate::git;
use crate::exit_codes;
use crate::colors::escseq::EscSeqFormat;
use crate::colors::get_config::get_formatter;
use crate::path_display;
use crate::jobs;
use crate::non_default_shell::non_default_shell_component;

#[derive(clap::Parser, Debug)]
/// Generate shell prompt
pub struct Prompt {
    /// Exit code (`$?`)
    exit_code: String,
    /// Jobs number
    jobs_number: String,
    /// The shell that's used and its formatting of escape sequences
    escformat: EscSeqFormat,
}

pub fn prompt(args: Prompt) {
    // Get current working directory
    let cwd = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let git_status = git::show::show_git_status(&args.escformat);
    let formatter = get_formatter(&args.escformat);
    print!(
        "{}{}{} {}{}",
        non_default_shell_component(&args.escformat),
        if &args.exit_code == "0" { "".to_string() } else { exit_codes::format_code(&args.escformat, &args.exit_code) },
        format!("{}{}", args.escformat.esc("1m"), formatter.format_str(
            &path_display::path_display_wrapper(&cwd),
        )),
        jobs::show_jobs(&args.escformat, &args.jobs_number),
        git_status,
    );
}
