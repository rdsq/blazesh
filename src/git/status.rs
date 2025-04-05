use std::process::Command;

#[derive(Debug)]
pub struct GitStatus {
    pub uncommitted: bool,
    pub upstream: bool,
    pub downstream: bool,
}

fn detect_changes(lines: &Vec<&str>) -> bool {
    for i in lines {
        if i.starts_with("?") || i.starts_with("1") {
            return true;
        }
    }
    false
}

fn detect_remote(lines: &Vec<&str>) -> [bool; 2] {
    for line in lines {
        if line.starts_with("# branch.ab ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                return [
                    parts[2].trim_start_matches('+') != "0",
                    parts[3].trim_start_matches('-') != "0",
                ];
            }
        }
    }
    [false, false]
}

pub fn git_status() -> Option<GitStatus> {
    let output = Command::new("git")
        .args(["status", "--porcelain=v2", "--branch"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.split('\n').collect();
    let uncommitted = detect_changes(&lines);
    let remote = detect_remote(&lines);

    Some(GitStatus {
        uncommitted,
        upstream: remote[0],
        downstream: remote[1],
    })
}
