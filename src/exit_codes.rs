use crate::colors::esc::color;

fn format_special(exit_code: &str) -> Option<String> {
    return match exit_code {
        "126" => Some("e403".to_string()),
        "127" => Some("e404".to_string()),
        "130" => Some("SIGINT".to_string()),
        "137" => Some("SIGKILL".to_string()),
        "143" => Some("SIGTERM".to_string()),
        _ => None,
    };
}

enum CodeConfig {
    Code,
    Message,
    Both,
}

const VAR_NAME: &str = "BLAZESH_EXIT_CODE_FORMAT";

fn get_config() -> CodeConfig {
    let val = std::env::var(VAR_NAME);
    if let Ok(mode) = val {
        return match &mode as &str {
            "code" => CodeConfig::Code,
            "message" => CodeConfig::Message,
            "both" => CodeConfig::Both,
            _ => {
                eprintln!("blazesh: unknown {} value", VAR_NAME);
                CodeConfig::Both
            }
        }
    }
    CodeConfig::Both // default
}

pub fn format_code(exit_code: &str) -> String {
    let message = match get_config() {
        CodeConfig::Code => exit_code.to_string(),
        CodeConfig::Message => format_special(exit_code).unwrap_or(exit_code.to_string()),
        CodeConfig::Both => {
            if exit_code.parse::<u8>().is_err() {
                format!("{}/fancy", exit_code)
            } else if let Some(special) = format_special(exit_code) {
                format!("{}/{}", exit_code, special)
            } else {
                exit_code.to_string()
            }
        },
    };
    format!(
        "{} ",
        color("31", &format!("[{}]", message)),
    )
}
