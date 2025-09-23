use super::status::git_status;
use super::modes::decide::{decide, Decision};
use crate::colors::escseq::EscSeqFormat;

fn construct_one_icon(escformat: &EscSeqFormat, symbol: &str, condition: &bool) -> String {
    if *condition {
        escformat.color("32;1", symbol)
    } else {
        escformat.color("2;1", symbol)
    }
}

fn static_git_status(escformat: &EscSeqFormat) -> String {
    format!("{} ", escformat.color("2;1", "[git]"))
}

pub fn get_updated_git_status(escformat: &EscSeqFormat) -> String {
    if let Some(status) = git_status() {
        let any_status = status.uncommitted || status.upstream || status.downstream;
        if any_status {
            format!(
                "{}{}{}{}{} ",
                escformat.color("34;1", "["),
                construct_one_icon(&escformat, "+", &status.uncommitted),
                construct_one_icon(&escformat, "↑", &status.upstream),
                construct_one_icon(&escformat, "↓", &status.downstream),
                escformat.color("34;1", "]"),
            )
        } else {
            static_git_status(&escformat)
        }
    } else {
        String::new()
    }
}

pub fn show_git_status(escformat: &EscSeqFormat) -> String {
    return match decide() {
        Decision::Status => get_updated_git_status(escformat),
        Decision::Static => static_git_status(escformat),
        Decision::Nothing => "".to_string(),
    }
}
