use crate::git_status::git_status;
use crate::esc::color;

fn construct_one_icon(symbol: &str, condition: &bool) -> String {
    if *condition {
        color("32;1", symbol)
    } else {
        color("2;1", symbol)
    }
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
            format!("{} ", color("2;1", "[git]"))
        }
    } else {
        String::new()
    }
}

pub fn show_git_status() -> String {
    get_updated_git_status()
}
