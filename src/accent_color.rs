use std::env;

pub fn get_accent_color() -> char {
    if let Some(custom_unchecked) = env::var_os("BLAZESH_ACCENT_COLOR") {
        if custom_unchecked.len() == 1 {
            let v = custom_unchecked.to_str()
                .unwrap_or(" ") // whatever
                .chars().nth(0).unwrap();
            // verifying
            if v == '0'
            || v == '1'
            || v == '2'
            || v == '3'
            || v == '4'
            || v == '5'
            || v == '6'
            || v == '7'
            || v == '9' {
                return v;
            }
        }
    }
    '6'
}
