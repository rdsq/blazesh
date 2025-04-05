pub fn get_blazesh_dir_representation() -> String {
    if let Some(blazesh_dir_os) = std::env::var_os("_BLAZESH_DIR") {
        if let Ok(blazesh_dir) = blazesh_dir_os.into_string() {
            return blazesh_dir;
        }
    }
    return "blazesh".to_string(); // not accurate, but it doesn't have to be 100% accurate
}
