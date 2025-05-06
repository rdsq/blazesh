use std::path::{PathBuf, Path};
use std::env;
use std::ffi::OsStr;

fn get_path_shorthands() -> Vec<(String, String)> {
    if let Ok(input) = env::var("BLAZESH_PATH_SHORTHANDS") {
        let mut result = Vec::new();
        for kv_input in input.split(';') {
            if kv_input.is_empty() {
                continue;
            }
            if let Some((k, v)) = kv_input.split_once(':') {
                result.push((k.to_string(), v.to_string()));
            } else {
                eprintln!("blazesh: failed to parse \"{}\" as key:value", kv_input);
            }
        }
        return result;
    } else if let Ok(home) = env::var("HOME") {
        return vec![(home, "~".to_string())]
    }
    Vec::new()
}

fn path_from_os_str(full_option: &Option<&OsStr>) -> String {
    if let Some(full) = full_option {
        if let Some(str_val) = full.to_str() {
            return str_val.to_string();
        }
    }
    "(somewhere broken)".to_string()
}

fn path_display(original_path: &str, depth: u8, is_first: bool, path_shorthands: &Vec<(String, String)>) -> String {
    let mut path = PathBuf::from(original_path);
    if path.parent().is_none() {
        // root dir `/`
        return if is_first { "/".to_string() } else { "".to_string() };
    }
    for shorthand in path_shorthands {
        if Path::new(&shorthand.0) == path.as_path() {
            return shorthand.1.to_string();
        }
    }
    if depth == 0 {
        return "...".to_string();
    }
    let basename = path_from_os_str(&path.file_name());
    path.pop();
    return format!(
        "{}/{}",
        path_display(&path.to_string_lossy(), depth - 1, false, path_shorthands),
        basename,
    );
}

const PATH_DEPTH: &str = "BLAZESH_PATH_DEPTH";

fn get_depth() -> u8 {
    if let Ok(depth_str) = std::env::var(PATH_DEPTH) {
        if let Ok(depth) = depth_str.parse::<u8>() {
            return depth;
        } else {
            eprintln!("blazesh: invalid {}: {}", PATH_DEPTH, depth_str);
        }
    }
    2 // default
}

pub fn path_display_wrapper(original_path: &str) -> String {
    path_display(
        original_path,
        get_depth(),
        true,
        &get_path_shorthands(),
    )
}
