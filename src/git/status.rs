use std::process::Command;

#[derive(Debug)]
pub struct GitStatus {
    pub uncommitted: u16,
    pub upstream: u16,
    pub downstream: u16,
}

fn detect_changes(lines: &Vec<&str>) -> u16 {
    let mut changes = 0;
    for i in lines {
        if i.starts_with("?") || i.starts_with("1") {
            changes = changes + 1;
        }
    }
    changes
}

fn detect_remote(lines: &Vec<&str>) -> (u16, u16) {
    for line in lines {
        if line.starts_with("# branch.ab ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                return (
                    parts[2].trim_start_matches('+').parse().unwrap_or(1),
                    parts[3].trim_start_matches('-').parse().unwrap_or(1),
                );
            }
        }
    }
    (0, 0)
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
        upstream: remote.0,
        downstream: remote.1,
    })
}

impl GitStatus {
    pub fn anything_going_on(&self) -> bool {
        self.downstream != 0
            || self.upstream != 0
            || self.uncommitted != 0
    }
}
