use super::check;
use super::get_config::{get_mode, Mode};
use std::env::current_dir;

pub enum Decision {
    Status,
    Static,
    Nothing,
}

pub fn decide() -> Decision {
    return match get_mode() {
        Mode::Unoptimized => Decision::Status,
        Mode::Optimized => {
            if let Ok(path) = current_dir() {
                if check::check_parent_dirs(&path) {
                    return Decision::Status;
                }
            }
            Decision::Nothing
        }
        Mode::OptimizedCwd => {
            if let Ok(path) = current_dir() {
                if check::check_dir(&path) {
                    return Decision::Status;
                }
            }
            Decision::Nothing
        },
        Mode::Static => {
            if let Ok(path) = current_dir() {
                if check::check_parent_dirs(&path) {
                    return Decision::Static;
                }
            }
            Decision::Nothing
        }
        Mode::StaticCwd => {
            if let Ok(path) = current_dir() {
                if check::check_dir(&path) {
                    return Decision::Static;
                }
            }
            Decision::Nothing
        },
        Mode::Disabled => Decision::Nothing,
    }
}
