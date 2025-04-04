use std::env;
use crate::git_status::git_status;
use crate::esc::color;

const CACHED_VAR_NAME: &str = "_BLAZESH_CACHED_GIT";

pub fn show_git_status() -> String {
    if let Some(env_var) = env::var_os(CACHED_VAR_NAME) {
        return env_var
            .to_str()
            .unwrap_or("")
            .to_string();
    } else {
        String::new()
    }
}

fn construct_one_icon(symbol: &str, condition: &bool) -> String {
    if *condition {
        color("32;1", symbol)
    } else {
        color("2;1", symbol)
    }
}

pub fn get_updated_git_status() -> String {
    if let Some(status) = git_status() {
        let brackets_color = if status.uncommitted || status.upstream || status.downstream { "34;1" } else { "2;1" };
        format!(
            "{}{}{}{}{} ",
            color(&brackets_color, "["),
            construct_one_icon("+", &status.uncommitted),
            construct_one_icon("↑", &status.upstream),
            construct_one_icon("↓", &status.downstream),
            color(&brackets_color, "]"),
        )
    } else {
        String::new()
    }
}

pub fn update_git_status(cached: &str) {
    let updated = get_updated_git_status();
    if updated != cached {
        unsafe {
            env::set_var(CACHED_VAR_NAME, updated);
        }
    }
}
