fn format_special(exit_code: &i32) -> String {
    return match exit_code {
        126 => "126/e403".to_string(),
        127 => "127/e404".to_string(),
        130 => "130/SIGINT".to_string(),
        137 => "137/SIGKILL".to_string(),
        143 => "143/SIGTERM".to_string(),
        n => {
            if *n < 0 || *n > 255 {
                return format!("{}/fancyy", n);
            }
            return n.to_string();
        },
    };
}

pub fn format_code(exit_code: &i32) -> String {
    format!("%{{\x1b[31m%}}[{}]%{{\x1b[0m%}} ", format_special(exit_code))
}
