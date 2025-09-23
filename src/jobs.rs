use crate::colors::escseq::EscSeqFormat;

pub fn show_jobs(escformat: &EscSeqFormat, jobs_number_input: &str) -> String {
    if jobs_number_input == "0" {
        return "".to_string();
    } else {
        format!(
            "{} ",
            escformat.color("36", &format!("[{}]", jobs_number_input)),
        )
    }
}
