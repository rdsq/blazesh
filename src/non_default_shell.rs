use std::env;
use crate::colors::escseq::EscSeqFormat;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn get_default_shell_from_env() -> Option<String> {
    // doesn't work as default shell in some environments
    // but rather current shell it seems, so not fully reliable
    env::var("SHELL").ok()
}

fn get_default_shell_from_fs() -> Option<String> {
    let mut user = env::var("USER").ok()?;
    user.push(':'); // passwd file separator
    let file = std::fs::File::open("/etc/passwd").ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        if line.starts_with(&user) {
            // found it
            let shell = line.split(':').nth(6)?; // the shell column
            return Some(shell.to_owned());
        }
    }
    None
}

fn get_default_shell() -> Option<String> {
    let raw_path = get_default_shell_from_fs()
        .or_else(|| get_default_shell_from_env())?;
    let path = Path::new(&raw_path);
    let fname_osstr = path.file_name()?;
    return Some(fname_osstr.to_str()?.to_owned());
}

pub fn non_default_shell_component(escformat: &EscSeqFormat) -> String {
    // it can be disabled
    if env::var("BLAZESH_NON_DEFAULT_SHELL") != Ok(String::from("disabled")) {
        if let Some(default_shell) = get_default_shell() {
            let current_shell = escformat.to_str();
            if current_shell != &default_shell {
                return format!("{} ", escformat.color("33", &format!("({})", current_shell)));
            }
        }
    }
    String::from("")
}
