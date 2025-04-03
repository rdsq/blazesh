use std::env;

fn main() {
    // Get current working directory
    let cwd = env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Construct the prompt
    let prompt = format!(
        "\x1b[36;1m{}\x1b[0m $ ",
        cwd,
    );

    print!("{}", prompt);
}
