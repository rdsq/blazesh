use crate::esc::esc_sequence as esc;

pub fn format_colors(colors: &Vec<char>, text: &str) -> String {
    let num_items = colors.len();
    if num_items == 0 {
        return text.to_string();
    }
    if num_items == 1 {
        // do not waste any stuff
        return format!(
            "{}{}{}",
            esc(&format!("3{}m", colors[0])),
            text,
            esc("0m"),
        );
    }
    let mut variation = 0;
    let mut res = String::new();
    for ch in text.chars() {
        res.push_str(&format!(
            "{}{}",
            esc(&format!("3{}m", &colors[variation])),
            ch,
        ));
        variation = variation + 1;
        if variation > (num_items - 1) {
            variation = 0; // reset
        }
    }
    res.push_str(&esc("0m"));
    res
}
