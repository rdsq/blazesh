use std::env;
mod cli;
mod exit_codes;
mod path_display;
mod git_status;
mod esc;
mod show_git_status;

fn main() {
    let args = cli::parse_args();
    // Get current working directory
    let cwd = env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let cached_git_status = show_git_status::show_git_status();
    print!(
        "{}%{{\x1b[36;1m%}}{}%{{\x1b[0m%}} {}$ ",
        if args.exit_code == 0 { "".to_string() } else { exit_codes::format_code(&args.exit_code) },
        path_display::path_display(&cwd, 2),
        cached_git_status,
    );
    show_git_status::update_git_status(&cached_git_status);
}
