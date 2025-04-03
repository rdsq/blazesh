use std::path::Path;
use std::env;
use std::ffi::OsStr;

fn basename_from_os_str(full_option: &Option<&OsStr>) -> String {
    if let Some(full) = full_option {
        if let Some(str_val) = full.to_str() {
            return str_val.to_string();
        }
    }
    "(somewhere broken)".to_string()
}

pub fn path_display(original_path: &str) -> String {
    let path = Path::new(original_path);
    if path.parent().is_none() {
        // root dir `/`
        return "/".to_string();
    }
    if let Some(home) = env::var_os("HOME") {
        if Path::new(&home) == path {
            return "~".to_string();
        }
    }
    // basic last dir
    basename_from_os_str(&path.file_name())
}
