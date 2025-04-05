use std::env;
mod cli;
mod exit_codes;
mod path_display;
mod jobs;
mod git;
mod dir_representation;
mod colors;

fn main() {
    let args = cli::parse_args();
    // Get current working directory
    let cwd = env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let git_status = git::show::show_git_status();
    let formatter = colors::get_config::get_formatter();
    print!(
        "{}{} {}{}",
        if args.exit_code == 0 { "".to_string() } else { exit_codes::format_code(&args.exit_code) },
        format!("{}{}", colors::esc::esc_sequence("1m"), formatter.format_str(
            &path_display::path_display_wrapper(&cwd),
        )),
        jobs::show_jobs(&args.jobs_number),
        git_status,
    );
}
