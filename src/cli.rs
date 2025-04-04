use std::env;
use std::process::exit;

pub struct ParsedArgs {
    pub exit_code: i32, // some systems support exit codes outside `u8`
    pub jobs_number: String,
}

pub fn parse_args() -> ParsedArgs {
    let mut args = env::args();
    if let (Some(code_str), Some(jobs_number)) = (args.nth(1), args.nth(2)) {
        let exit_code = code_str.parse::<i32>()
            .unwrap_or_else(|_err| {
                eprintln!("blazesh: fialed to parse exit code argument");
                exit(1);
            });
        ParsedArgs {
            exit_code,
            jobs_number,
        }
    } else {
        eprintln!("blazesh: not enough arguments");
        exit(1);
    }
}
