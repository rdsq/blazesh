use std::env;
use std::process::exit;

pub struct ParsedArgs {
    pub exit_code: i32, // some systems support exit codes outside `u8`
}

pub fn parse_args() -> ParsedArgs {
    if let Some(code_str) = env::args().nth(1) {
        let exit_code = code_str.parse::<i32>()
            .unwrap_or_else(|_err| {
                eprintln!("blazesh: fialed to parse exit code argument");
                exit(1);
            });
        ParsedArgs {
            exit_code,
        }
    } else {
        eprintln!("blazesh: not enough arguments");
        exit(1);
    }
}
