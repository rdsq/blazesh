use super::status::git_status;
use super::modes::decide::{decide, Decision};
use super::verbosity::GitVerbosityConfig;
use crate::colors::escseq::EscSeqFormat;

fn construct_one_icon(escformat: &EscSeqFormat, symbol: &str, num: u16, verbose: bool) -> String {
    if num != 0 {
        let content = if verbose && num != 1 {
            &format!("{}{}", symbol, num)
        } else {
            symbol
        };
        escformat.color("32;1", content)
    } else {
        escformat.color("2;1", symbol)
    }
}

fn static_git_status(escformat: &EscSeqFormat) -> String {
    format!("{} ", escformat.color("2;1", "[git]"))
}

pub fn get_updated_git_status(escformat: &EscSeqFormat) -> String {
    if let Some(status) = git_status() {
        if status.anything_going_on() {
            let verbosity = GitVerbosityConfig::detect();
            format!(
                "{}{}{}{}{} ",
                escformat.color("34;1", "["),
                construct_one_icon(&escformat, "+", status.uncommitted, verbosity.show_uncommitted()),
                construct_one_icon(&escformat, "↑", status.upstream, verbosity.show_remotes()),
                construct_one_icon(&escformat, "↓", status.downstream, verbosity.show_remotes()),
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
