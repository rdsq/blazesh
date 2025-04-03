use std::path::{PathBuf, Path};
use std::env;
use std::ffi::OsStr;

fn path_from_os_str(full_option: &Option<&OsStr>) -> String {
    if let Some(full) = full_option {
        if let Some(str_val) = full.to_str() {
            return str_val.to_string();
        }
    }
    "(somewhere broken)".to_string()
}

pub fn path_display(original_path: &str, depth: u8) -> String {
    let mut path = PathBuf::from(original_path);
    if path.parent().is_none() {
        // root dir `/`
        return "/".to_string();
    }
    if let Some(home) = env::var_os("HOME") {
        if Path::new(&home) == path.as_path() {
            return "~".to_string();
        }
    }
    if depth == 0 {
        return "...".to_string();
    }
    let basename = path_from_os_str(&path.file_name());
    path.pop();
    return format!(
        "{}/{}",
        path_display(&path.to_string_lossy(), depth - 1),
        basename,
    );
}
