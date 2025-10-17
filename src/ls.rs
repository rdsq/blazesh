use crate::colors::escseq::EscSeqFormat;

fn parse_ls_colors_variable<'a>(var_val: &'a Option<String>) -> (&'a str, &'a str) {
    // defaults
    let mut di = "34;1";
    let mut fi = "0";
    if let Some(val) = var_val.as_ref() {
        for piece in val.split(':') {
            if let Some((kw, col_val)) = piece.split_once('=') {
                match kw {
                    "di" => di = col_val,
                    "fi" => fi = col_val,
                    _ => {}, // ignore
                }
            }
        }
    }
    (di, fi)
}

#[derive(PartialEq)]
enum DotsMode {
    Ignored,
    Counted,
    Separate,
}

fn get_config() -> (bool, DotsMode) {
    let mut enabled = false;
    let mut dots_mode = DotsMode::Separate;
    if let Ok(conf_var) = std::env::var("BLAZESH_LS") {
        for piece in conf_var.split(';') {
            match piece {
                // I know this code is horrible
                "enabled" => enabled = true,
                "disabled" => enabled = false, // useless since it is the default
                "dots=ignored" => dots_mode = DotsMode::Ignored,
                "dots=counted" => dots_mode = DotsMode::Counted,
                "dots=separate" => dots_mode = DotsMode::Separate,
                x => eprintln!("blazesh: unknown BLAZESH_LS value: {}", x),
            }
        }
    }
    (enabled, dots_mode)
}

fn wrap_exp(exp: &str, escformat: &EscSeqFormat) -> String {
    format!(
        "{}{}{} ",
        escformat.color("36", "("),
        exp,
        escformat.color("36", ")"),
    )
}

pub fn ls_component(escformat: &EscSeqFormat, cwd: &str) -> String {
    let (enabled, dots_mode) = get_config();
    if !enabled {
        return "".to_owned();
    }
    if let Ok(dir_contents) = std::fs::read_dir(cwd) {
        let mut dirs: usize = 0;
        let mut files: usize = 0;
        let mut dots: usize = 0;
        for entry in dir_contents {
            if let Ok(entry) = entry {
                if entry.file_name().to_string_lossy().starts_with('.') {
                    match dots_mode {
                        DotsMode::Ignored => continue,
                        DotsMode::Counted => {},
                        DotsMode::Separate => { dots += 1; continue },
                    }
                }
                if entry.path().is_dir() {
                    dirs += 1;
                } else {
                    files += 1;
                }
            }
        }
        let var_original_so_rust_wont_complain = std::env::var("LS_COLORS").ok();
        let cols = parse_ls_colors_variable(&var_original_so_rust_wont_complain);
        let dirs_processed = escformat.color(cols.0, &dirs.to_string());
        let files_processed = escformat.color(cols.1, &files.to_string());
        return wrap_exp(&if dots_mode == DotsMode::Separate {format!(
            "{} {} {}",
            dirs_processed,
            files_processed,
            escformat.color("2", &dots.to_string()),
        )} else {format!(
            "{} {}",
            dirs_processed,
            files_processed,
        )}, escformat);
    } else {
        return wrap_exp("err", escformat);
    }
}
