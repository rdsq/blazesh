use crate::git::status::git_status;
use crate::colors::esc::color;
use crate::git::modes::decide::{decide, Decision};

fn construct_one_icon(symbol: &str, condition: &bool) -> String {
    if *condition {
        color("32;1", symbol)
    } else {
        color("2;1", symbol)
    }
}

fn static_git_status() -> String {
    format!("{} ", color("2;1", "[git]"))
}

pub fn get_updated_git_status() -> String {
    if let Some(status) = git_status() {
        let any_status = status.uncommitted || status.upstream || status.downstream;
        if any_status {
            format!(
                "{}{}{}{}{} ",
                color("34;1", "["),
                construct_one_icon("+", &status.uncommitted),
                construct_one_icon("↑", &status.upstream),
                construct_one_icon("↓", &status.downstream),
                color("34;1", "]"),
            )
        } else {
            static_git_status()
        }
    } else {
        String::new()
    }
}

pub fn show_git_status() -> String {
    return match decide() {
        Decision::Status => get_updated_git_status(),
        Decision::Static => static_git_status(),
        Decision::Nothing => "".to_string(),
    }
}
