use std::env;

pub struct ParsedArgs {
    pub exit_code: i32, // some systems support exit codes outside `u8`
    pub jobs_number: String,
}

fn get_exit_code_arg(arg: Option<String>) -> i32 {
    if let Some(code_str) = arg {
        if let Ok(exit_code) = code_str.parse::<i32>() {
            return exit_code;
        } else {
            eprintln!("blazesh: failed to parse exit code from \"{}\"", code_str);
        }
    } else {
        eprintln!("blazesh: exit code argument not provided");
    }
    // synthetic default
    0
}

fn get_jobs_arg(arg: Option<String>) -> String {
    if let Some(jobs_number) = arg {
        return jobs_number;
    } else {
        eprintln!("blazesh: jobs number argument not provided");
    }
    return "0".to_string();
}

pub fn parse_args() -> ParsedArgs {
    let mut args = env::args();
    ParsedArgs {
        exit_code: get_exit_code_arg(args.nth(1)),
        jobs_number: get_jobs_arg(args.nth(2)),
    }
}
