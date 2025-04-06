pub fn get_blazesh_dir_representation() -> String {
    if let Ok(blazesh_dir) = std::env::var("_BLAZESH_DIR") {
        return blazesh_dir;
    } else {
        // not accurate, but it doesn't have to be 100% accurate
        // it's just a representation for the user
        return "blazesh".to_string();
    }
}
