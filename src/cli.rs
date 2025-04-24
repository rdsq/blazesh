use std::env;

#[derive(Debug)]
pub struct ParsedArgs {
    pub exit_code: String,
    pub jobs_number: String,
}

fn get_exit_code_arg(arg: Option<String>) -> String {
    if let Some(exit_code) = arg {
        return exit_code;
    } else {
        eprintln!("blazesh: exit code argument not provided");
    }
    // synthetic default
    "0".to_string()
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
    args.next();
    ParsedArgs {
        exit_code: get_exit_code_arg(args.next()),
        jobs_number: get_jobs_arg(args.next()),
    }
}
