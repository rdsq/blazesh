use std::env;
use std::process::exit;

pub struct ParsedArgs {
    pub exit_code: u8,
}

pub fn parse_args() -> ParsedArgs {
    if let Some(code_str) = env::args().nth(1) {
        let exit_code = code_str.parse::<u8>()
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
