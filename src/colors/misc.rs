use ansi_colours::ansi256_from_rgb;
use rgb::RGB8;

fn try_parse_rgb(rgb_str: &str) -> Option<RGB8> {
    let mut sp = rgb_str.split(',');
    if let (Some(r_str), Some(g_str), Some(b_str)) = (sp.next(), sp.next(), sp.next()) {
        return Some(RGB8 {
            r: r_str.parse().ok()?,
            g: g_str.parse().ok()?,
            b: b_str.parse().ok()?,
        });
    }
    None
}

fn try_parse_hex(hex: &str) -> Option<RGB8> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(RGB8 { r, g, b })
}

pub fn try_parse_col(col: &str) -> Option<RGB8> {
    if let Some(res) = try_parse_hex(col) {
        return Some(res);
    }
    if let Some(res) = try_parse_rgb(col) {
        return Some(res);
    }
    None
}

pub fn to_ansi_foreground(rgb: &RGB8) -> String {
    format!("\x1b[38;5;{}m", ansi256_from_rgb(rgb))
}
