use std::env;
use crate::colors::escseq::EscSeqFormat;
use std::path::Path;

fn get_default_shell() -> Option<String> {
    let raw_path = env::var("SHELL").ok()?;
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
