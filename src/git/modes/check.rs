use std::path::Path;

pub fn check_dir(path: &Path) -> bool {
    let mut buf = path.to_path_buf();
    buf.push(".git");
    return buf.is_dir();
}

pub fn check_parent_dirs(path: &Path) -> bool {
    let mut buf = path.to_path_buf();
    loop {
        if check_dir(&buf) {
            return true;
        }
        if !buf.pop() {
            // kind of do while loop
            break;
        }
    }
    false
}
