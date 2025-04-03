use std::env;
mod cli;

fn format_code(exit_code: &i32) -> String {
    format!("\x1b[31m[{}]\x1b[0m ", exit_code)
}

fn main() {
    let args = cli::parse_args();
    // Get current working directory
    let cwd = env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    print!(
        "{}\x1b[36;1m{}\x1b[0m $ ",
        if args.exit_code == 0 { "".to_string() } else { format_code(&args.exit_code) },
        cwd,
    );
}
