use std::env;

fn verify_and_extract(chunk: &str) -> Option<char> {
    if chunk.len() == 1 {
        let v = chunk
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
            return Some(v);
        }
    }
    None
}

pub fn get_accent_color() -> Vec<char> {
    if let Some(custom_unchecked) = env::var_os("BLAZESH_ACCENT_COLOR") {
        if let Some(as_str) = custom_unchecked.to_str() {
            let mut res: Vec<char> = Vec::new();
            let sp = as_str.split_whitespace();
            for chunk in sp {
                if let Some(ch) = verify_and_extract(chunk) {
                    res.push(ch);
                } else {
                    eprintln!("blazesh: invalid color code at BLAZESH_ACCENT_COLOR: {}", chunk);
                }
            }
            return res;
        }
    }
    vec!['6']
}
