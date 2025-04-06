use std::env;
use crate::dir_representation::get_blazesh_dir_representation;

pub enum Mode {
    Unoptimized,
    Optimized,
    OptimizedCwd,
    Static,
    StaticCwd,
    Disabled,
}

pub fn get_mode() -> Mode {
    let conf_possible = env::var("BLAZESH_GIT_MODE").ok();
    if conf_possible.is_none() {
        return Mode::Optimized; // default
    }
    let conf: &str = &conf_possible.unwrap();
    return match conf {
        "unoptimized" => Mode::Unoptimized,
        "optimized" => Mode::Optimized,
        "optimized-cwd" => Mode::OptimizedCwd,
        "static" => Mode::Static,
        "static-cwd" => Mode::StaticCwd,
        "disabled" => Mode::Disabled,
        // spellings
        "unoptimised" => Mode::Unoptimized,
        "optimised" => Mode::Optimized,
        "optimised-cwd" => Mode::OptimizedCwd,
        // default
        _ => {
            eprintln!("blazesh: unknown value of the BLAZESH_GIT_MODE variable: {}", conf);
            eprintln!("Check {}/README.md#Configuration for more info", get_blazesh_dir_representation());
            return Mode::Optimized;
        },
    }
}
