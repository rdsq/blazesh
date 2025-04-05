use crate::colors::esc::color;

pub fn show_jobs(jobs_number_input: &str) -> String {
    if jobs_number_input == "0" {
        return "".to_string();
    } else {
        format!(
            "{} ",
            color("36", &format!("[{}]", jobs_number_input)),
        )
    }
}
