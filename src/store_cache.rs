use std::fs;
use std::env;
use std::path::PathBuf;

fn get_cache_path(filename: &str) -> PathBuf {
    let tmp_dir = env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(tmp_dir).join(filename)
}

pub fn save_cache(filename: &str, data: &str) {
    let path = get_cache_path(filename);
    if let Err(e) = fs::write(&path, data) {
        eprintln!("Failed to write cache: {}", e);
    }
}

pub fn load_cache(filename: &str) -> Option<String> {
    let path = get_cache_path(filename);
    fs::read_to_string(&path).ok()
}
