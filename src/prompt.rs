use crate::cli::parse_args;
use crate::git;
use crate::exit_codes;
use crate::colors;
use crate::path_display;
use crate::jobs;

pub fn prompt() {
    let args = parse_args();
    // Get current working directory
    let cwd = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let git_status = git::show::show_git_status();
    let formatter = colors::get_config::get_formatter();
    print!(
        "{}{} {}{}",
        if &args.exit_code == "0" { "".to_string() } else { exit_codes::format_code(&args.exit_code) },
        format!("{}{}", colors::esc::esc_sequence("1m"), formatter.format_str(
            &path_display::path_display_wrapper(&cwd),
        )),
        jobs::show_jobs(&args.jobs_number),
        git_status,
    );
}
